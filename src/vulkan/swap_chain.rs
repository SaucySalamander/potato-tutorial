use ash::extensions::khr::Swapchain;
use ash::vk::{
    Extent2D, Format, Image, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SwapchainKHR, PhysicalDevice
};
use ash::{Instance, Device};
use super::surface::PotatoSurface;
use super::queue_family::QueueFamily;

pub struct PotatoSwapChain {
    swapchain_loader: Swapchain,
    swapchain: SwapchainKHR,
    swapcahin_images: Vec<Image>,
    swapchain_format: Format,
    swapchain_exten: Extent2D,
}

pub struct SwapChainSupportDetail {
    capabilities: SurfaceCapabilitiesKHR,
    formats: Vec<SurfaceFormatKHR>,
    present_modes: Vec<PresentModeKHR>,
}

fn create_swapchain(insatnce: &Instance, device: &Device, physical_device: PhysicalDevice,surface: PotatoSurface,queue_family: QueueFamily) -> PotatoSwapChain {
    //TODO create function to figure out if device support swapchain creation
    let swapchain_support = determine_swapchain_support(physical_device, surface);

    let surface_format = choose_swapchain_format(&swapchain_support.formats);
    let present_mode = choose_swapchain_present_mode(&swapchain_support.present_modes);
    let extent = choose_swapchain_extent(&swapchain_support.capabilities);

}

fn determine_swapchain_support(physical_device: PhysicalDevice, surface: PotatoSurface) -> SwapChainSupportDetail {
    unsafe {
        let capabilities = surface.surface_loader.get_physical_device_surface_capabilities(physical_device, surface).expect("Failed to find surface capabilities");
        let formats = surface.surface_loader.get_physical_device_surface_formats(physical_device, surface).expect("Failed to find surface formats");
        let present_modes = surface.surface_loader.get_physical_device_surface_present_modes(physical_device, surface).expect("Failed to find surface present modes");
        SwapChainSupportDetail{
            capabilities,
            formats,
            present_modes,
        }
    }
}