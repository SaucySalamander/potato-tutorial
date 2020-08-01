use ash::vk::{QueueFamilyProperties, QueueFlags};
use log::{info};

pub struct QueueFamily {
    pub graphics_family: Option<usize>,
}

impl QueueFamily {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

pub fn find_graphical_queue_family(queue_family_properties: &[QueueFamilyProperties]) -> QueueFamily {
    let graphics_family = queue_family_properties
        .iter()
        .position(|x| x.queue_count > 0 && x.queue_flags.contains(QueueFlags::GRAPHICS));

    info!("Position of graphical queue family {}", graphics_family.unwrap());

    QueueFamily{
        graphics_family
    }
}
