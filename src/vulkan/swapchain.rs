use super::queue_family::QueueFamily;
use super::surface::PotatoSurface;
use ash::extensions::khr::Swapchain;
use ash::vk::{
    ColorSpaceKHR, Extent2D, Format, Image, PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR,
    SurfaceFormatKHR, SwapchainKHR, SharingMode, SwapchainCreateInfoKHR, StructureType, SwapchainCreateFlagsKHR, ImageUsageFlags, CompositeAlphaFlagsKHR, TRUE
};
use ash::{Device, Instance};
use num::clamp;

pub struct PotatoSwapChain {
    pub swapchain_loader: Swapchain,
    pub swapchain: SwapchainKHR,
    pub swapchain_images: Vec<Image>,
    pub swapchain_format: Format,
    pub swapchain_extent: Extent2D,
}

pub struct SwapChainSupportDetail {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>,
}

pub fn create_swapchain(
    instance: &Instance,
    device: &Device,
    physical_device: PhysicalDevice,
    surface: &PotatoSurface,
    _queue_family: &QueueFamily,
) -> PotatoSwapChain {
    let swapchain_support = determine_swapchain_support(physical_device, surface);

    let surface_format = choose_swapchain_format(&swapchain_support.formats);
    let present_mode = choose_swapchain_present_mode(&swapchain_support.present_modes);
    let extent = choose_swapchain_extent(&swapchain_support.capabilities);

    let image_count = if swapchain_support.capabilities.max_image_count > 0 {
        swapchain_support.capabilities.max_image_count
    } else {
        swapchain_support.capabilities.min_image_count + 1
    };

    let (image_sharing_mode, queue_family_index_count, queue_family_indices) = (SharingMode::EXCLUSIVE, 0, vec![]);

    let swapchain_create_info = SwapchainCreateInfoKHR {
        s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        p_next: std::ptr::null(),
        flags: SwapchainCreateFlagsKHR::empty(),
        surface: surface.surface,
        min_image_count: image_count,
        image_color_space: surface_format.color_space,
        image_format: surface_format.format,
        image_extent: extent,
        image_usage: ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode,
        p_queue_family_indices: queue_family_indices.as_ptr(),
        queue_family_index_count,
        pre_transform: swapchain_support.capabilities.current_transform,
        composite_alpha: CompositeAlphaFlagsKHR::OPAQUE,
        present_mode,
        clipped: TRUE,
        old_swapchain: SwapchainKHR::null(),
        image_array_layers: 1,
    };

    let swapchain_loader = Swapchain::new(instance, device);
    let swapchain = unsafe {
        swapchain_loader.create_swapchain(&swapchain_create_info, None).expect("Failed to create swapchain")
    };

    let swapchain_images = unsafe {
        swapchain_loader.get_swapchain_images(swapchain).expect("Failed to get swapchain images")
    };

    PotatoSwapChain{
        swapchain_loader,
        swapchain,
        swapchain_format: surface_format.format,
        swapchain_extent: extent,
        swapchain_images,
    }
}

pub fn determine_swapchain_support(
    physical_device: PhysicalDevice,
    surface: &PotatoSurface,
) -> SwapChainSupportDetail {
    unsafe {
        let capabilities = surface
            .surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface.surface)
            .expect("Failed to find surface capabilities");
        let formats = surface
            .surface_loader
            .get_physical_device_surface_formats(physical_device, surface.surface)
            .expect("Failed to find surface formats");
        let present_modes = surface
            .surface_loader
            .get_physical_device_surface_present_modes(physical_device, surface.surface)
            .expect("Failed to find surface present modes");
        SwapChainSupportDetail {
            capabilities,
            formats,
            present_modes,
        }
    }
}

fn choose_swapchain_format(available_foramts: &Vec<SurfaceFormatKHR>) -> SurfaceFormatKHR {
    available_foramts
        .iter()
        .find(|x| {
            x.format == Format::B8G8R8A8_SRGB && x.color_space == ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or(available_foramts.first().unwrap())
        .to_owned()
}

fn choose_swapchain_present_mode(available_present_modes: &Vec<PresentModeKHR>) -> PresentModeKHR {
    available_present_modes
        .iter()
        .find(|x| **x == PresentModeKHR::MAILBOX)
        .unwrap_or(&PresentModeKHR::FIFO)
        .to_owned()
}

fn choose_swapchain_extent(capabilities: &SurfaceCapabilitiesKHR) -> Extent2D {
    if capabilities.current_extent.width != u32::max_value() {
        capabilities.current_extent
    } else {
        Extent2D {
            width: clamp(
                800,
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ),
            height: clamp(
                600,
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ),
        }
    }
}
