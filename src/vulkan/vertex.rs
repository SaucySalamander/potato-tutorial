use super::buffer::{copy_buffer, create_buffer};
use super::constants::{VERTICES_DATA,INDICES_DATA};
use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk::{
    Buffer, BufferUsageFlags, CommandPool, DeviceMemory, DeviceSize, Format, MemoryMapFlags,
    MemoryPropertyFlags, PhysicalDevice, Queue, VertexInputAttributeDescription,
    VertexInputBindingDescription, VertexInputRate,
};
use ash::Device;
use ash::Instance;
use memoffset::offset_of;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn get_binding_descriptions() -> [VertexInputBindingDescription; 1] {
        [VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: VertexInputRate::VERTEX,
        }]
    }

    pub fn get_attribute_descriptions() -> [VertexInputAttributeDescription; 2] {
        [
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: Format::R32G32_SFLOAT,
                offset: offset_of!(Self, pos) as u32,
            },
            VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: Format::R32G32B32_SFLOAT,
                offset: offset_of!(Self, color) as u32,
            },
        ]
    }
}

pub fn create_vertex_buffer(
    instance: &Instance,
    device: &Device,
    physical_device: PhysicalDevice,
    command_pool: CommandPool,
    submit_queue: Queue,
    buffer_usage_flags: BufferUsageFlags,
) -> (Buffer, DeviceMemory) {
    let buffer_size = std::mem::size_of_val(&VERTICES_DATA) as DeviceSize;
    let device_memory_properties =
        unsafe { instance.get_physical_device_memory_properties(physical_device) };

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        device,
        buffer_size,
        BufferUsageFlags::TRANSFER_SRC,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        &device_memory_properties,
    );
    unsafe {
        let data_ptr = device
            .map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                MemoryMapFlags::empty(),
            )
            .expect("Failed to map memory") as *mut Vertex;

        data_ptr.copy_from_nonoverlapping(VERTICES_DATA.as_ptr(), VERTICES_DATA.len());

        device.unmap_memory(staging_buffer_memory);
    }

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        device,
        buffer_size,
        buffer_usage_flags,
        MemoryPropertyFlags::DEVICE_LOCAL,
        &device_memory_properties,
    );

    copy_buffer(
        device,
        submit_queue,
        command_pool,
        staging_buffer,
        vertex_buffer,
        buffer_size,
    );

    unsafe {
        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);
    }

    (vertex_buffer, vertex_buffer_memory)
}

pub fn create_index_buffer(
    instance: &Instance,
    device: &Device,
    physical_device: PhysicalDevice,
    command_pool: CommandPool,
    submit_queue: Queue,
    buffer_usage_flags: BufferUsageFlags,
) -> (Buffer, DeviceMemory) {
    let buffer_size = std::mem::size_of_val(&INDICES_DATA) as DeviceSize;
    let device_memory_properties =
        unsafe { instance.get_physical_device_memory_properties(physical_device) };

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        device,
        buffer_size,
        BufferUsageFlags::TRANSFER_SRC,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        &device_memory_properties,
    );
    unsafe {
        let data_ptr = device
            .map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                MemoryMapFlags::empty(),
            )
            .expect("Failed to map memory") as *mut u32;

        data_ptr.copy_from_nonoverlapping(INDICES_DATA.as_ptr(), INDICES_DATA.len());

        device.unmap_memory(staging_buffer_memory);
    }

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        device,
        buffer_size,
        buffer_usage_flags,
        MemoryPropertyFlags::DEVICE_LOCAL,
        &device_memory_properties,
    );

    copy_buffer(
        device,
        submit_queue,
        command_pool,
        staging_buffer,
        vertex_buffer,
        buffer_size,
    );
    unsafe {
        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);
    }

    (vertex_buffer, vertex_buffer_memory)
}
