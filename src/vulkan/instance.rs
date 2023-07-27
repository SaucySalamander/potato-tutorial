use super::constants::VALIDATION;
use super::utilities::{conver_str_vec_to_c_str_ptr_vec, vk_to_string};
use super::vulk_validation_layers::populate_debug_messenger_create_info;
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
#[cfg(feature = "wayland")]
use ash::extensions::khr::WaylandSurface;
#[cfg(feature = "xlib")]
use ash::extensions::khr::XlibSurface;
use ash::vk::{
    make_api_version, ApplicationInfo, DebugUtilsMessengerCreateInfoEXT, InstanceCreateFlags,
    InstanceCreateInfo, StructureType,
};
use ash::Entry;
use ash::Instance;
use log::debug;
use std::ffi::CString;
use std::os::raw::c_void;

pub fn create_instance(entry: &Entry) -> Instance {
    if VALIDATION.is_enable && !check_validation_layer_support(entry) {
        panic!("Validation layers requested but not supported");
    }

    let app_name = CString::new("Potato").unwrap();
    let engine_name = CString::new("Vulkan API").unwrap();
    let app_info = ApplicationInfo {
        s_type: StructureType::APPLICATION_INFO,
        p_next: std::ptr::null(),
        p_application_name: app_name.as_ptr(),
        application_version: make_api_version(0, 0, 0, 1),
        p_engine_name: engine_name.as_ptr(),
        engine_version: make_api_version(0, 0, 0, 1),
        api_version: make_api_version(0, 1, 2, 148),
    };

    let debug_utils_create_info = populate_debug_messenger_create_info();

    let extension_names = create_extention_names();

    let (cstring_vec, enable_layer_names) =
        conver_str_vec_to_c_str_ptr_vec(VALIDATION.required_validation_layers.to_vec());

    debug!("{:?}", cstring_vec);
    let create_info = InstanceCreateInfo {
        s_type: StructureType::INSTANCE_CREATE_INFO,
        p_next: if VALIDATION.is_enable {
            &debug_utils_create_info as *const DebugUtilsMessengerCreateInfoEXT as *const c_void
        } else {
            std::ptr::null()
        },
        flags: InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        pp_enabled_layer_names: if VALIDATION.is_enable {
            enable_layer_names.as_ptr()
        } else {
            std::ptr::null()
        },
        enabled_layer_count: get_enabled_layers_len(),
        pp_enabled_extension_names: extension_names.as_ptr(),
        enabled_extension_count: extension_names.len() as u32,
    };

    debug!("Creating Instance with {:?}", create_info);
    let instance: Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create instance")
    };
    debug!("Finished creating instance");
    instance
}

fn check_validation_layer_support(entry: &Entry) -> bool {
    let layer_properties = entry
        .enumerate_instance_layer_properties()
        .expect("Failed to enumerate the Instance Layers Properties!");

    debug!("{:?}", layer_properties);
    VALIDATION
        .required_validation_layers
        .iter()
        .map(|layers| {
            layer_properties
                .iter()
                .any(|v| vk_to_string(&v.layer_name) == *layers)
        })
        .any(|b| b)
}

fn get_enabled_layers_len() -> u32 {
    if VALIDATION.is_enable {
        VALIDATION.required_validation_layers.iter().len() as u32
    } else {
        0 as u32
    }
}

#[cfg(feature = "wayland")]
fn create_extention_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
        WaylandSurface::name().as_ptr(),
    ]
}

#[cfg(feature = "xlib")]
fn create_extention_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}
