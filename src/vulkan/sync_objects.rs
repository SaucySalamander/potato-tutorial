use super::constants::MAX_FRAMES_IN_FLIGHT;
use ash::vk::{
    Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateFlags, SemaphoreCreateInfo,
    StructureType,
};
use ash::Device;

pub struct SyncObjects {
    pub image_available_semaphores: Vec<Semaphore>,
    pub render_finished_semaphores: Vec<Semaphore>,
    pub inflight_fences: Vec<Fence>,
}

pub fn create_sync_objects(device: &Device) -> SyncObjects {
    let mut sync_objects = SyncObjects {
        image_available_semaphores: vec![],
        render_finished_semaphores: vec![],
        inflight_fences: vec![],
    };

    let semaphore_create_info = SemaphoreCreateInfo {
        s_type: StructureType::SEMAPHORE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: SemaphoreCreateFlags::empty(),
    };

    let fence_create_info = FenceCreateInfo {
        s_type: StructureType::FENCE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: FenceCreateFlags::SIGNALED,
    };

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        unsafe {
            let image_available_semaphore = device
                .create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create Semaphore Object!");
            let render_finished_semaphore = device
                .create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create Semaphore Object!");
            let inflight_fence = device
                .create_fence(&fence_create_info, None)
                .expect("Failed to create Fence Object!");

            sync_objects
                .image_available_semaphores
                .push(image_available_semaphore);
            sync_objects
                .render_finished_semaphores
                .push(render_finished_semaphore);
            sync_objects.inflight_fences.push(inflight_fence);
        }
    }

    sync_objects
}
