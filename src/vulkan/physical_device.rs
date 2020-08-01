use super::queue_family::find_graphical_queue_family;
use super::utilities::vk_to_string;
use ash::version::InstanceV1_0;
use ash::vk::{
    version_major, version_minor, version_patch, PhysicalDevice, PhysicalDeviceProperties,
    PhysicalDeviceType, QueueFlags,
};
use ash::Instance;
use log::info;

pub fn select_physical_device(instance: &Instance) -> PhysicalDevice {
    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
    };
    info!("{} GPU device(s) found", physical_devices.len());

    let selected_device = physical_devices.iter().find(|x| {
        let device_queue_familes =
            unsafe { instance.get_physical_device_queue_family_properties(**x) };
        let family = find_graphical_queue_family(&device_queue_familes);
        family.is_complete()
    });

    describe_device(&instance, *selected_device.unwrap());
    
    *selected_device.unwrap()
}

fn describe_device(instance: &Instance, physical_device: PhysicalDevice) {
    let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let device_queue_familes =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let device_type = find_device_type(&device_properties);
    let device_name = vk_to_string(&device_properties.device_name);

    info!(
        "Device Name: {}, id: {}, type: {}",
        device_name, device_properties.device_id, device_type
    );

    info!(
        "Supported API Version {}.{}.{}",
        version_major(device_properties.api_version),
        version_minor(device_properties.api_version),
        version_patch(device_properties.api_version)
    );

    device_queue_familes.iter().for_each(|x| {
        info!("Queue Count, Graphics, Compute, Transfer, Sparse Binding");
        info!(
            "{}, {}, {}, {}, {}",
            x.queue_count,
            x.queue_flags.contains(QueueFlags::GRAPHICS),
            x.queue_flags.contains(QueueFlags::COMPUTE),
            x.queue_flags.contains(QueueFlags::TRANSFER),
            x.queue_flags.contains(QueueFlags::SPARSE_BINDING)
        );
    });
}

fn find_device_type(properties: &PhysicalDeviceProperties) -> &str {
    match properties.device_type {
        PhysicalDeviceType::CPU => "CPU",
        PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
        PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
        PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
        _ => panic!(),
    }
}
