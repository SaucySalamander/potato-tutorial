use ash::Device;
use ash::Instance;
use ash::version::InstanceV1_0;
use ash::version::DeviceV1_0;
use ash::vk::{PhysicalDevice, StructureType, DeviceQueueCreateFlags, DeviceQueueCreateInfo, PhysicalDeviceFeatures, DeviceCreateInfo, DeviceCreateFlags};
use ash::extensions::khr::Swapchain;
use super::queue_family::{find_graphical_queue_family, QueueFamily};
use super::utilities::conver_str_vec_to_c_str_ptr_vec;
use super::constants::VALIDATION;
use super::surface::PotatoSurface;
use log::debug;

pub fn create_logical_device(instance: &Instance, physical_device: PhysicalDevice, surface: &PotatoSurface) -> (Device, QueueFamily){
    let queue_family = find_graphical_queue_family(instance, physical_device, surface);

    let queue_priorities = [1.0_f32];

    let queue_create_info = DeviceQueueCreateInfo {
        s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: DeviceQueueCreateFlags::empty(),
        queue_family_index: queue_family.graphics_family.unwrap() as u32, 
        p_queue_priorities: queue_priorities.as_ptr(),
        queue_count: queue_priorities.len() as u32,
    };

    let physical_device_features = PhysicalDeviceFeatures {
        ..Default::default()
    };

    let (cstring_vec, enable_layer_names) = conver_str_vec_to_c_str_ptr_vec(VALIDATION.required_validation_layers.to_vec());
    debug!("{:?}", cstring_vec);

    let enable_extension_names = [
        Swapchain::name().as_ptr(),
    ];

    let device_create_info = DeviceCreateInfo {
        s_type: StructureType::DEVICE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: DeviceCreateFlags::empty(),
        queue_create_info_count: 1,
        p_queue_create_infos: &queue_create_info,
        enabled_layer_count: if VALIDATION.is_enable {
            enable_layer_names.len()
        } else {
            0
        } as u32,
        pp_enabled_layer_names: if VALIDATION.is_enable {
            enable_layer_names.as_ptr()
        } else {
            std::ptr::null()
        },
        enabled_extension_count: enable_extension_names.len() as u32,
        pp_enabled_extension_names: enable_extension_names.as_ptr(),
        p_enabled_features: &physical_device_features,
    };

    let device: Device = unsafe {
        instance.create_device(physical_device, &device_create_info, None).expect("Failed to create logical device")
    };

    let _graphics_queue = unsafe {
        device.get_device_queue(queue_family.graphics_family.unwrap() as u32, 0) 
    };

    (device, queue_family)
}