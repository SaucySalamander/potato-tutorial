use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::QueueFamily;
use vulkano::device::{Device, DeviceExtensions, QueuesIter};
use vulkano::swapchain::{Surface, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain};
use vulkano::image::SwapchainImage;

use winit::window::{Window};

pub fn create_instance() -> Arc<Instance> {
    let required_extensions = vulkano_win::required_extensions();
    Instance::new(None, &required_extensions, None).unwrap()
}

pub fn create_physical_device(instance: &Arc<Instance>) -> PhysicalDevice {
    PhysicalDevice::enumerate(instance).next().unwrap()
}

pub fn setup_device(phsyical_device: &PhysicalDevice, queue_family: &QueueFamily) -> (Arc<Device>, QueuesIter){
    let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
    Device::new(*phsyical_device, phsyical_device.supported_features(), &device_ext, [(*queue_family, 0.5)].iter().cloned()).unwrap()
}

pub fn create_swapchain(surface: &Arc<Surface<Window>>, physical_device: &PhysicalDevice) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>){
    let queue_family = physical_device.queue_families().find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
    .unwrap();

    let (device, mut queues) = setup_device(&physical_device, &queue_family);
    let queue = queues.next().unwrap();

    let caps = surface.capabilities(*physical_device).unwrap();
    let usage = caps.supported_usage_flags;
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;
    let dimensions: [u32; 2] = surface.window().inner_size().into();

    Swapchain::new(
        device,
        surface.clone(),
        caps.min_image_count,
        format,
        dimensions,
        1,
        usage,
        &queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        FullscreenExclusive::Default,
        true,
        ColorSpace::SrgbNonLinear,
    )
    .unwrap()
}