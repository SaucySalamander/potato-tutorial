use super::constants::INDICES_DATA;
use super::queue_family::QueueFamily;
use ash::vk::{
    Buffer, ClearColorValue, ClearValue, CommandBuffer, CommandBufferAllocateInfo,
    CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsageFlags, CommandPool,
    CommandPoolCreateFlags, CommandPoolCreateInfo, DescriptorSet, Extent2D, Framebuffer, IndexType,
    Offset2D, Pipeline, PipelineBindPoint, PipelineLayout, Rect2D, RenderPass, RenderPassBeginInfo,
    StructureType, SubpassContents,
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

//TODO Reduce number of arguments
pub fn create_command_buffers(
    device: &Device,
    command_pool: CommandPool,
    graphics_pipeline: Pipeline,
    framebuffers: &[Framebuffer],
    render_pass: RenderPass,
    surface_extent: Extent2D,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline_layout: PipelineLayout,
    descriptor_sets: &Vec<DescriptorSet>,
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
            vertex_buffer,
            index_buffer,
            pipeline_layout,
            descriptor_sets,
        )
    });

    command_buffers
}

//TODO Reduce number of arguments
fn process_command_buffer(
    index: usize,
    command_buffer: &CommandBuffer,
    render_pass: RenderPass,
    framebuffers: &[Framebuffer],
    surface_extent: Extent2D,
    device: &Device,
    graphics_pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline_layout: PipelineLayout,
    descriptor_sets: &Vec<DescriptorSet>,
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
        let vertex_buffers = [vertex_buffer];
        let offsets = [0_u64];
        let descriptor_sets_to_bind = [descriptor_sets[index]];
        device.cmd_bind_vertex_buffers(*command_buffer, 0, &vertex_buffers, &offsets);
        device.cmd_bind_index_buffer(*command_buffer, index_buffer, 0, IndexType::UINT32);
        device.cmd_bind_descriptor_sets(
            *command_buffer,
            PipelineBindPoint::GRAPHICS,
            pipeline_layout,
            0,
            &descriptor_sets_to_bind,
            &[],
        );
        device.cmd_draw_indexed(*command_buffer, INDICES_DATA.len() as u32, 1, 0, 0, 0);
        device.cmd_end_render_pass(*command_buffer);
        device
            .end_command_buffer(*command_buffer)
            .expect("Failed to record command buffer ending");
    }
}
