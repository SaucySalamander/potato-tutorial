use crate::io::file::read_file_to_bytes;
use ash::version::DeviceV1_0;
use ash::vk::{
    PipelineShaderStageCreateFlags, PipelineShaderStageCreateInfo, ShaderModule,
    ShaderModuleCreateFlags, ShaderModuleCreateInfo, ShaderStageFlags, StructureType,
};
use ash::Device;
use std::ffi::CString;

pub fn create_graphics_pipeline(device: &Device) {
    let vert_shader = read_file_to_bytes("src/shaders/spv/shader-vert.spv").unwrap();
    let frag_shader = read_file_to_bytes("src/shaders/spv/shader-frag.spv").unwrap();

    let vert_module = create_shader_module(device, vert_shader);
    let frag_module = create_shader_module(device, frag_shader);

    let main_function_name = CString::new("main").unwrap();

    let shader_stages = [
        PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: PipelineShaderStageCreateFlags::empty(),
            module: vert_module,
            p_name: main_function_name.as_ptr(),
            p_specialization_info: std::ptr::null(),
            stage: ShaderStageFlags::VERTEX,
        },
        PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: PipelineShaderStageCreateFlags::empty(),
            module: frag_module,
            p_name: main_function_name.as_ptr(),
            p_specialization_info: std::ptr::null(),
            stage: ShaderStageFlags::FRAGMENT,
        },
    ];

    unsafe {
        device.destroy_shader_module(vert_module, None);
        device.destroy_shader_module(frag_module, None);
    }
}

fn create_shader_module(device: &Device, code: Vec<u8>) -> ShaderModule {
    let shader_module_create_info = ShaderModuleCreateInfo {
        s_type: StructureType::SHADER_MODULE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: ShaderModuleCreateFlags::empty(),
        code_size: code.len(),
        p_code: code.as_ptr() as *const u32,
    };

    unsafe {
        device
            .create_shader_module(&shader_module_create_info, None)
            .expect("Failed to create shader module")
    }
}
