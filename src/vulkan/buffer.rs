use ash::version::DeviceV1_0;
use ash::vk::{
    Buffer, BufferCreateFlags, BufferCreateInfo, BufferUsageFlags, CommandBufferAllocateInfo,
    CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsageFlags, CommandPool, DeviceMemory,
    DeviceSize, MemoryAllocateInfo, MemoryPropertyFlags, PhysicalDeviceMemoryProperties, Queue,
    SharingMode, StructureType, SubmitInfo, Fence, BufferCopy
};
use ash::Device;

pub fn create_buffer(
    device: &Device,
    size: DeviceSize,
    usage: BufferUsageFlags,
    required_memory_properties: MemoryPropertyFlags,
    device_memory_properties: &PhysicalDeviceMemoryProperties,
) -> (Buffer, DeviceMemory) {
    let buffer_create_info = BufferCreateInfo {
        s_type: StructureType::BUFFER_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: BufferCreateFlags::empty(),
        size,
        usage,
        sharing_mode: SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: std::ptr::null(),
    };

    let buffer = unsafe {
        device
            .create_buffer(&buffer_create_info, None)
            .expect("Failed to create Vertex Buffer")
    };

    let mem_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

    let memory_type = find_mem_type(
        mem_requirements.memory_type_bits,
        required_memory_properties,
        *device_memory_properties,
    );

    let allocate_info = MemoryAllocateInfo {
        s_type: StructureType::MEMORY_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        allocation_size: mem_requirements.size,
        memory_type_index: memory_type,
    };

    let buffer_memory = unsafe {
        device
            .allocate_memory(&allocate_info, None)
            .expect("Failed to allocate vertex buffer memory")
    };

    unsafe {
        device
            .bind_buffer_memory(buffer, buffer_memory, 0)
            .expect("Failed to bind buffer");
    }

    (buffer, buffer_memory)
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
        })
        .unwrap() as u32
}

pub fn copy_buffer(
    device: &Device,
    submit_queue: Queue,
    command_pool: CommandPool,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    size: DeviceSize,
) {
    let allocate_info = CommandBufferAllocateInfo {
        s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        command_buffer_count: 1,
        command_pool,
        level: CommandBufferLevel::PRIMARY,
    };

    let command_buffers = unsafe {
        device
            .allocate_command_buffers(&allocate_info)
            .expect("Failed to allocate command buffer")
    };

    let begin_info = CommandBufferBeginInfo {
        s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: std::ptr::null(),
        flags: CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        p_inheritance_info: std::ptr::null(),
    };

    unsafe {
        device
            .begin_command_buffer(command_buffers[0], &begin_info)
            .expect("Failed to begin command buffer");
        let copy_regions = [BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size,
        }];
        device.cmd_copy_buffer(command_buffers[0], src_buffer, dst_buffer, &copy_regions);
        device
            .end_command_buffer(command_buffers[0])
            .expect("Failed to end command buffer");
    }

    let submit_info = [SubmitInfo {
        s_type: StructureType::SUBMIT_INFO,
        p_next: std::ptr::null(),
        wait_semaphore_count: 0,
        p_wait_semaphores: std::ptr::null(),
        p_wait_dst_stage_mask: std::ptr::null(),
        command_buffer_count: 1,
        p_command_buffers: &command_buffers[0],
        signal_semaphore_count: 0,
        p_signal_semaphores: std::ptr::null(),
    }];

    //TODO add logic for semaphore or fence for multiple submissions.
    unsafe {
        device
            .queue_submit(submit_queue, &submit_info, Fence::null())
            .expect("Failed to submit queue");
        device
            .queue_wait_idle(submit_queue)
            .expect("Failed to wait on queue");
        device.free_command_buffers(command_pool, &command_buffers);
    }
}
