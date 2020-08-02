use super::surface::PotatoSurface;
use ash::version::InstanceV1_0;
use ash::vk::{PhysicalDevice, QueueFlags};
use ash::Instance;
use log::info;

pub struct QueueFamily {
    pub graphics_family: Option<usize>,
    pub present_family: Option<usize>,
}

impl QueueFamily {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

pub fn find_graphical_queue_family(
    instance: &Instance,
    physical_device: PhysicalDevice,
    surface: &PotatoSurface,
) -> QueueFamily {
    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let graphics_family = queue_families
        .iter()
        .position(|x| x.queue_count > 0 && x.queue_flags.contains(QueueFlags::GRAPHICS));

    info!(
        "Position of graphical queue family {}",
        graphics_family.unwrap()
    );

    let is_present_supported = unsafe {
        surface.surface_loader.get_physical_device_surface_support(
            physical_device,
            graphics_family.unwrap() as u32,
            surface.surface,
        )
    };

    if queue_families[graphics_family.unwrap()].queue_count > 0 && is_present_supported.unwrap() {
        QueueFamily {
            graphics_family,
            present_family: graphics_family,
        }
    } else {
        panic!("Could not find a graphical queue that also supports surface present");
    }
}
