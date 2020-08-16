use super::constants::VERTICES_DATA;
use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk::{
    Buffer, BufferCreateFlags, BufferCreateInfo, BufferUsageFlags, DeviceMemory, Format,
    MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags, PhysicalDevice,
    PhysicalDeviceMemoryProperties, SharingMode, StructureType, VertexInputAttributeDescription,
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
) -> (Buffer, DeviceMemory) {
    let vertex_buffer_create_info = BufferCreateInfo {
        s_type: StructureType::BUFFER_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: BufferCreateFlags::empty(),
        size: std::mem::size_of_val(&VERTICES_DATA) as u64,
        usage: BufferUsageFlags::VERTEX_BUFFER,
        sharing_mode: SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: std::ptr::null(),
    };

    let vertex_buffer = unsafe {
        device
            .create_buffer(&vertex_buffer_create_info, None)
            .expect("Failed to create Vertex Buffer")
    };

    let mem_requirements = unsafe { device.get_buffer_memory_requirements(vertex_buffer) };

    let mem_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };

    let required_mem_flags = MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT;

    let memory_type = find_mem_type(
        mem_requirements.memory_type_bits,
        required_mem_flags,
        mem_properties,
    );

    let allocate_info = MemoryAllocateInfo {
        s_type: StructureType::MEMORY_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        allocation_size: mem_requirements.size,
        memory_type_index: memory_type,
    };

    let vertex_buffer_memory = unsafe {
        device
            .allocate_memory(&allocate_info, None)
            .expect("Failed to allocate vertex buffer memory")
    };

    unsafe {
        device
            .bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0)
            .expect("Failed to bind buffer");

        let data_ptr = device
            .map_memory(
                vertex_buffer_memory,
                0,
                vertex_buffer_create_info.size,
                MemoryMapFlags::empty(),
            )
            .expect("Failed to map memory") as *mut Vertex;

        data_ptr.copy_from_nonoverlapping(VERTICES_DATA.as_ptr(), VERTICES_DATA.len());

        device.unmap_memory(vertex_buffer_memory);
    }

    (vertex_buffer, vertex_buffer_memory)
}

fn find_mem_type(
    type_filter: u32,
    required_properties: MemoryPropertyFlags,
    mem_properties: PhysicalDeviceMemoryProperties,
) -> u32 {
    mem_properties
        .memory_types
        .iter()
        .enumerate()
        .position(|(i, x)| {
            type_filter & (1 << i) > 0 && x.property_flags.contains(required_properties)
        }).unwrap() as u32
}
