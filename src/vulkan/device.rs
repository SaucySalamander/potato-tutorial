use ash::Device;
use ash::Instance;
use ash::version::InstanceV1_0;
use ash::version::DeviceV1_0;
use ash::vk::{PhysicalDevice, Queue, StructureType, DeviceQueueCreateFlags, DeviceQueueCreateInfo, PhysicalDeviceFeatures, DeviceCreateInfo, DeviceCreateFlags};
use super::queue_family::find_graphical_queue_family;
use super::utilities::conver_str_vec_to_c_str_ptr_vec;
use super::vulk_validation_layers::VALIDATION;

pub fn create_logical_device(instance: &Instance, physical_device: PhysicalDevice) -> (Device, Queue){
    let queue_family_properties = unsafe {instance.get_physical_device_queue_family_properties(physical_device)};
    let queue_family = find_graphical_queue_family(&queue_family_properties);

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
        enabled_extension_count: 0,
        pp_enabled_extension_names: std::ptr::null(),
        p_enabled_features: &physical_device_features,
    };

    let device: Device = unsafe {
        instance.create_device(physical_device, &device_create_info, None).expect("Failed to create logical device")
    };

    let graphics_queue = unsafe {
        device.get_device_queue(queue_family.graphics_family.unwrap() as u32, 0) 
    };

    (device, graphics_queue)
}