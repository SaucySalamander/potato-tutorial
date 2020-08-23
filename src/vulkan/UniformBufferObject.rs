use super::buffer::create_buffer;
use super::swapchain::PotatoSwapChain;
use ash::version::DeviceV1_0;
use ash::vk::{
    Buffer, BufferUsageFlags, DescriptorSetLayout, DescriptorSetLayoutBinding,
    DescriptorSetLayoutCreateFlags, DescriptorSetLayoutCreateInfo, DescriptorType, DeviceMemory,
    MemoryPropertyFlags, PhysicalDeviceMemoryProperties, ShaderStageFlags, StructureType, MemoryMapFlags
};
use ash::Device;
use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct UniformBufferObject {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>,
}

pub fn create_descriptor_set_layout(device: &Device) -> DescriptorSetLayout {
    let ubo_layout_bindings = [DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: DescriptorType::UNIFORM_BUFFER,
        descriptor_count: 1,
        stage_flags: ShaderStageFlags::VERTEX,
        p_immutable_samplers: std::ptr::null(),
    }];

    let ubo_layout_create_info = DescriptorSetLayoutCreateInfo {
        s_type: StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: DescriptorSetLayoutCreateFlags::empty(),
        binding_count: ubo_layout_bindings.len() as u32,
        p_bindings: ubo_layout_bindings.as_ptr(),
    };

    unsafe {
        device
            .create_descriptor_set_layout(&ubo_layout_create_info, None)
            .expect("Failed to create Descriptor set layout")
    }
}

pub fn create_uniform_buffers(
    device: &Device,
    device_memory_properties: &PhysicalDeviceMemoryProperties,
    swapchain_image_count: usize,
) -> (Vec<Buffer>, Vec<DeviceMemory>) {
    let buffer_size = std::mem::size_of::<UniformBufferObject>();
    let mut uniform_buffers = vec![];
    let mut uniform_buffers_memory = vec![];

    for _ in 0..swapchain_image_count {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            device,
            buffer_size as u64,
            BufferUsageFlags::UNIFORM_BUFFER,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            device_memory_properties,
        );
        uniform_buffers.push(uniform_buffer);
        uniform_buffers_memory.push(uniform_buffer_memory);
    }

    (uniform_buffers, uniform_buffers_memory)
}

pub fn update_uniform_buffer(
    swapchain: &PotatoSwapChain,
    device: &Device,
    current_image: usize,
    delta_time: f32,
    uniform_buffers_memory: &Vec<DeviceMemory>,
) {
    let ubos = [UniformBufferObject {
        model: Matrix4::from_angle_z(Deg(90.0 * delta_time)),
        view: Matrix4::look_at(
            Point3::new(2.0, 2.0, 2.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ),
        proj: perspective(
            Deg(45.0),
            swapchain.swapchain_extent.width as f32 / swapchain.swapchain_extent.height as f32,
            0.1,
            10.0,
        ),
    }];

    let buffer_size = (std::mem::size_of::<UniformBufferObject>() * ubos.len()) as u64;

    unsafe {
        let data_ptr = device
            .map_memory(
                uniform_buffers_memory[current_image],
                0,
                buffer_size,
                MemoryMapFlags::empty(),
            )
            .expect("Failed to map memory") as *mut UniformBufferObject;
        
            data_ptr.copy_from_nonoverlapping(ubos.as_ptr(), ubos.len());

            device.unmap_memory(uniform_buffers_memory[current_image]);
    }
}
