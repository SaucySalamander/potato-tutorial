use ash::vk::{
    Extent2D, Framebuffer, FramebufferCreateFlags, FramebufferCreateInfo, ImageView, RenderPass,
    StructureType,
};
use ash::Device;

pub fn create_framebuffers(
    device: &Device,
    render_pass: RenderPass,
    image_views: &[ImageView],
    swapchain_extent: &Extent2D,
) -> Vec<Framebuffer> {
    image_views
        .iter()
        .map(|x| {
            let attachments = [*x];
            FramebufferCreateInfo {
                s_type: StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: FramebufferCreateFlags::empty(),
                render_pass,
                attachment_count: attachments.len() as u32,
                p_attachments: attachments.as_ptr(),
                width: swapchain_extent.width,
                height: swapchain_extent.height,
                layers: 1,
            }
        })
        .map(|x| unsafe {
            device
                .create_framebuffer(&x, None)
                .expect("Failed to create framebuffer")
        })
        .collect()
}
