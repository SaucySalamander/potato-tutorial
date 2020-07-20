use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::QueueFamily;
use vulkano::device::{Device, DeviceExtensions, QueuesIter};


fn create_instance() -> Arc<Instance> {
    let required_extensions = vulkano_win::required_extensions();
    Instance::new(None, &required_extensions, None).unwrap()
}

fn create_physical_device(instance: &Arc<Instance>) -> PhysicalDevice {
    PhysicalDevice::enumerate(instance).next().unwrap()
}

fn setup_device(phsyical_device: &PhysicalDevice, queue_family: &QueueFamily) -> (Arc<Device>, QueuesIter){
    let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
    Device::new(*phsyical_device, phsyical_device.supported_features(), &device_ext, [(*queue_family, 0.5)].iter().cloned()).unwrap()
}