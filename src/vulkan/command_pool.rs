use super::queue_family::QueueFamily;
use ash::version::DeviceV1_0;
use ash::vk::{
    ClearColorValue, ClearValue, CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel,
    CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, Extent2D, Framebuffer, Offset2D,
    Pipeline, PipelineBindPoint, Rect2D, RenderPass, RenderPassBeginInfo, StructureType,
    SubpassContents, CommandBufferUsageFlags, CommandBufferBeginInfo
};
use ash::Device;

pub fn create_command_pool(device: &Device, queue_familes: &QueueFamily) -> CommandPool {
    let command_pool_create_info = CommandPoolCreateInfo {
        s_type: StructureType::COMMAND_POOL_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: CommandPoolCreateFlags::empty(),
        queue_family_index: queue_familes.graphics_family.unwrap() as u32,
    };

    unsafe {
        device
            .create_command_pool(&command_pool_create_info, None)
            .expect("Failed to create Command Pool")
    }
}

pub fn create_command_buffers(
    device: &Device,
    command_pool: CommandPool,
    graphics_pipeline: Pipeline,
    framebuffers: &[Framebuffer],
    render_pass: RenderPass,
    surface_extent: Extent2D,
) -> Vec<CommandBuffer> {
    let command_buffer_allocate_info = CommandBufferAllocateInfo {
        s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        command_buffer_count: framebuffers.len() as u32,
        command_pool,
        level: CommandBufferLevel::PRIMARY,
    };

    let command_buffers = unsafe {
        device
            .allocate_command_buffers(&command_buffer_allocate_info)
            .expect("failed to create command buffers")
    };

    command_buffers.iter().enumerate().for_each(|(i, x)| {
        process_command_buffer(
            i,
            x,
            render_pass,
            framebuffers,
            surface_extent,
            device,
            graphics_pipeline,
        )
    });

    command_buffers
}

fn process_command_buffer(
    index: usize,
    command_buffer: &CommandBuffer,
    render_pass: RenderPass,
    framebuffers: &[Framebuffer],
    surface_extent: Extent2D,
    device: &Device,
    graphics_pipeline: Pipeline,
) {
    let command_buffer_begin_info = CommandBufferBeginInfo {
        s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: std::ptr::null(),
        p_inheritance_info: std::ptr::null(),
        flags: CommandBufferUsageFlags::SIMULTANEOUS_USE,
    };

    unsafe {
        device
            .begin_command_buffer(*command_buffer, &command_buffer_begin_info)
            .expect("Failed to begin recording Command Buffer at beginning!");
    }

    let clear_values = [ClearValue {
        color: ClearColorValue {
            float32: [0.0, 0.0, 0.0, 1.0],
        },
    }];

    let render_pass_begin_info = RenderPassBeginInfo {
        s_type: StructureType::RENDER_PASS_BEGIN_INFO,
        p_next: std::ptr::null(),
        render_pass,
        framebuffer: framebuffers[index],
        render_area: Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: surface_extent,
        },
        clear_value_count: clear_values.len() as u32,
        p_clear_values: clear_values.as_ptr(),
    };

    unsafe {
        device.cmd_begin_render_pass(
            *command_buffer,
            &render_pass_begin_info,
            SubpassContents::INLINE,
        );
        device.cmd_bind_pipeline(
            *command_buffer,
            PipelineBindPoint::GRAPHICS,
            graphics_pipeline,
        );
        device.cmd_draw(*command_buffer, 3, 1, 0, 0);
        device.cmd_end_render_pass(*command_buffer);
        device
            .end_command_buffer(*command_buffer)
            .expect("Failed to record command buffer ending");
    }
}
