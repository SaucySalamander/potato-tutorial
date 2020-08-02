pub struct ValidationInfo {
    pub is_enable: bool,
    pub required_validation_layers: [&'static str; 1],
}

pub const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
};

pub struct DeviceExtension {
    pub names: [&'static str; 1]
}

pub const DEVICE_EXTENSTIONS: DeviceExtension = DeviceExtension {
    names: ["VK_KHR_swapchain"],
};