use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn vk_to_string(raw_string_array: &[c_char]) -> String {
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string
        .to_str()
        .expect("Failed to convert vulkan raw string.")
        .to_owned()
}

pub fn conver_str_vec_to_c_str_ptr_vec(vec: Vec<&str>) -> (Vec<CString>, Vec<*const i8>) {
    let requred_validation_layer_raw_names: Vec<CString> = vec
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();
    let enable_layer_names: Vec<*const i8> = requred_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    (requred_validation_layer_raw_names, enable_layer_names)
}
