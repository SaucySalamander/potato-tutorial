use super::constants::DEVICE_EXTENSTIONS;
use super::queue_family::find_graphical_queue_family;
use super::surface::PotatoSurface;
use super::swapchain::determine_swapchain_support;
use super::utilities::vk_to_string;
use ash::vk::{
    api_version_major, api_version_minor, api_version_patch, PhysicalDevice,
    PhysicalDeviceProperties, PhysicalDeviceType, QueueFlags,
};
use ash::Instance;
use log::{debug, info};
use std::collections::HashSet;

pub fn select_physical_device(instance: &Instance, surface: &PotatoSurface) -> PhysicalDevice {
    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
    };
    info!("{} GPU device(s) found", physical_devices.len());

    let selected_device = physical_devices
        .iter()
        .find(|x| check_device_compatability(instance, **x, surface));

    debug!("{:?}", selected_device);
    match selected_device {
        Some(p_physical_device) => *p_physical_device,
        None => panic!("Failed to find compatable device"),
    }
}

fn check_device_compatability(
    instance: &Instance,
    physical_device: PhysicalDevice,
    surface: &PotatoSurface,
) -> bool {
    let queue_family_support = is_queue_family_supported(instance, physical_device, surface);
    let device_extension_support = is_device_extension_supported(instance, physical_device);
    let swapchain_support =
        is_swapchain_supported(device_extension_support, physical_device, surface);

    debug!(
        "{}, {}, {}",
        queue_family_support, device_extension_support, swapchain_support
    );
    queue_family_support && device_extension_support && swapchain_support
}

fn is_queue_family_supported(
    instance: &Instance,
    physical_device: PhysicalDevice,
    surface: &PotatoSurface,
) -> bool {
    let queue_family = find_graphical_queue_family(instance, physical_device, surface);
    queue_family.is_complete()
}

fn is_device_extension_supported(instance: &Instance, physical_device: PhysicalDevice) -> bool {
    let available_extensions = unsafe {
        instance
            .enumerate_device_extension_properties(physical_device)
            .expect("Failed to get device extension properties")
    };

    debug!("Available Extensions");
    available_extensions.iter().for_each(|x| {
        debug!(
            "Name: {}, Version: {}",
            vk_to_string(&x.extension_name),
            x.spec_version
        )
    });

    let required_extensions: HashSet<String> = DEVICE_EXTENSTIONS
        .names
        .iter()
        .map(|x| x.to_string())
        .collect();

    required_extensions
        .iter()
        .map(|x| {
            available_extensions
                .iter()
                .map(|y| vk_to_string(&y.extension_name) == *x)
                .any(|z| z)
        })
        .any(|a| a)
}

fn is_swapchain_supported(
    device_extension_support: bool,
    physical_device: PhysicalDevice,
    surface: &PotatoSurface,
) -> bool {
    if device_extension_support {
        let available_support = determine_swapchain_support(physical_device, surface);
        !available_support.formats.is_empty() && !available_support.present_modes.is_empty()
    } else {
        false
    }
}

pub fn describe_device(instance: &Instance, physical_device: PhysicalDevice) {
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
        api_version_major(device_properties.api_version),
        api_version_minor(device_properties.api_version),
        api_version_patch(device_properties.api_version)
    );

    device_queue_familes.iter().for_each(|x| {
        debug!("Queue Count, Graphics, Compute, Transfer, Sparse Binding");
        debug!(
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
