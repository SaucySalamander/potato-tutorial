use ash::version::DeviceV1_0;
use ash::vk::{
    AccessFlags, AttachmentDescription, AttachmentDescriptionFlags, AttachmentLoadOp,
    AttachmentReference, AttachmentStoreOp, DependencyFlags, Format, ImageLayout,
    PipelineBindPoint, PipelineStageFlags, RenderPass, RenderPassCreateFlags, RenderPassCreateInfo,
    SampleCountFlags, StructureType, SubpassDependency, SubpassDescription,
    SubpassDescriptionFlags, SUBPASS_EXTERNAL,
};
use ash::Device;

pub fn create_render_pass(device: &Device, surface_format: Format) -> RenderPass {
    let color_attachment = AttachmentDescription {
        flags: AttachmentDescriptionFlags::empty(),
        format: surface_format,
        samples: SampleCountFlags::TYPE_1,
        load_op: AttachmentLoadOp::CLEAR,
        store_op: AttachmentStoreOp::STORE,
        stencil_load_op: AttachmentLoadOp::DONT_CARE,
        stencil_store_op: AttachmentStoreOp::DONT_CARE,
        initial_layout: ImageLayout::UNDEFINED,
        final_layout: ImageLayout::PRESENT_SRC_KHR,
    };

    let color_attachment_ref = AttachmentReference {
        attachment: 0,
        layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };

    let subpass = SubpassDescription {
        flags: SubpassDescriptionFlags::empty(),
        pipeline_bind_point: PipelineBindPoint::GRAPHICS,
        input_attachment_count: 0,
        p_input_attachments: std::ptr::null(),
        color_attachment_count: 1,
        p_color_attachments: &color_attachment_ref,
        p_resolve_attachments: std::ptr::null(),
        p_depth_stencil_attachment: std::ptr::null(),
        preserve_attachment_count: 0,
        p_preserve_attachments: std::ptr::null(),
    };

    let render_pass_attachments = [color_attachment];

    let subpass_dependencies = [SubpassDependency {
        src_subpass: SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: AccessFlags::empty(),
        dst_access_mask: AccessFlags::COLOR_ATTACHMENT_WRITE,
        dependency_flags: DependencyFlags::empty(),
    }];

    let render_pass_create_info = RenderPassCreateInfo {
        s_type: StructureType::RENDER_PASS_CREATE_INFO,
        flags: RenderPassCreateFlags::empty(),
        p_next: std::ptr::null(),
        attachment_count: render_pass_attachments.len() as u32,
        p_attachments: render_pass_attachments.as_ptr(),
        subpass_count: 1,
        p_subpasses: &subpass,
        dependency_count: subpass_dependencies.len() as u32,
        p_dependencies: subpass_dependencies.as_ptr(),
    };

    unsafe {
        device
            .create_render_pass(&render_pass_create_info, None)
            .expect("Failed to create render pass")
    }
}
