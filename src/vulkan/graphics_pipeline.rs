use super::vertex::Vertex;
use crate::io::file::read_file_to_bytes;
use ash::version::DeviceV1_0;
use ash::vk::{
    BlendFactor, BlendOp, ColorComponentFlags, CompareOp, CullModeFlags, Extent2D, FrontFace,
    GraphicsPipelineCreateInfo, LogicOp, Offset2D, Pipeline, PipelineCache,
    PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateFlags,
    PipelineColorBlendStateCreateInfo, PipelineCreateFlags, PipelineDepthStencilStateCreateFlags,
    PipelineDepthStencilStateCreateInfo, PipelineInputAssemblyStateCreateFlags,
    PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateFlags,
    PipelineLayoutCreateInfo, PipelineMultisampleStateCreateFlags,
    PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateFlags,
    PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateFlags,
    PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateFlags,
    PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateFlags,
    PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, Rect2D, RenderPass,
    SampleCountFlags, ShaderModule, ShaderModuleCreateFlags, ShaderModuleCreateInfo,
    ShaderStageFlags, StencilOp, StencilOpState, StructureType, VertexInputAttributeDescription,
    VertexInputBindingDescription, Viewport, FALSE,
};
use ash::Device;
use std::ffi::CString;

pub fn create_graphics_pipeline(
    device: &Device,
    render_pass: RenderPass,
    swapchain_extent: Extent2D,
) -> (Pipeline, PipelineLayout) {
    let vert_shader = read_file_to_bytes("src/shaders/spv/shader-vert.spv").unwrap();
    let frag_shader = read_file_to_bytes("src/shaders/spv/shader-frag.spv").unwrap();

    let vert_module = create_shader_module(device, vert_shader);
    let frag_module = create_shader_module(device, frag_shader);

    let main_function_name = CString::new("main").unwrap();

    let shader_stages = [
        PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: PipelineShaderStageCreateFlags::empty(),
            module: vert_module,
            p_name: main_function_name.as_ptr(),
            p_specialization_info: std::ptr::null(),
            stage: ShaderStageFlags::VERTEX,
        },
        PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: PipelineShaderStageCreateFlags::empty(),
            module: frag_module,
            p_name: main_function_name.as_ptr(),
            p_specialization_info: std::ptr::null(),
            stage: ShaderStageFlags::FRAGMENT,
        },
    ];

    let binding_description = Vertex::get_binding_descriptions();
    let attribute_description = Vertex::get_attribute_descriptions();

    let vertex_input_state_create_info =
        create_vertex_input_state_create_info(&attribute_description, &binding_description);
    let vertex_input_assembly_state_info = create_vertex_input_assembly_state_info();

    let viewports = create_viewport(&swapchain_extent);
    let scissors = create_scissors(&swapchain_extent);

    let viewport_state_create_info = create_viewport_state_create_info(&viewports, &scissors);
    let rasterization_state_create_info = create_rasterization_state_create_info();
    let multisample_state_create_info = create_multisample_state_create_info();

    let stencil_state = create_stencil_state();

    let depth_state_create_info = create_depth_state_create_info(&stencil_state);
    let color_blend_attachment_states = create_color_blend_attachment_states();

    let color_blend_state = create_color_blend_state(&color_blend_attachment_states);

    let pipeline_layout_create_info = create_pipeline_layout_create_info();

    let pipeline_layout = unsafe {
        device
            .create_pipeline_layout(&pipeline_layout_create_info, None)
            .expect("Failed to create pipeline layout")
    };

    let graphics_pipeline_create_infos = [GraphicsPipelineCreateInfo {
        s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: PipelineCreateFlags::empty(),
        stage_count: shader_stages.len() as u32,
        p_stages: shader_stages.as_ptr(),
        p_vertex_input_state: &vertex_input_state_create_info,
        p_input_assembly_state: &vertex_input_assembly_state_info,
        p_tessellation_state: std::ptr::null(),
        p_viewport_state: &viewport_state_create_info,
        p_rasterization_state: &rasterization_state_create_info,
        p_multisample_state: &multisample_state_create_info,
        p_depth_stencil_state: &depth_state_create_info,
        p_color_blend_state: &color_blend_state,
        p_dynamic_state: std::ptr::null(),
        layout: pipeline_layout,
        render_pass,
        subpass: 0,
        base_pipeline_handle: Pipeline::null(),
        base_pipeline_index: -1,
    }];

    let graphics_pipelines = unsafe {
        device
            .create_graphics_pipelines(PipelineCache::null(), &graphics_pipeline_create_infos, None)
            .expect("Failed to create graphics pipelines")
    };
    unsafe {
        device.destroy_shader_module(vert_module, None);
        device.destroy_shader_module(frag_module, None);
    }

    (graphics_pipelines[0], pipeline_layout)
}

fn create_shader_module(device: &Device, code: Vec<u8>) -> ShaderModule {
    #[allow(clippy::cast_ptr_alignment)]
    let shader_module_create_info = ShaderModuleCreateInfo {
        s_type: StructureType::SHADER_MODULE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: ShaderModuleCreateFlags::empty(),
        code_size: code.len(),
        p_code: code.as_ptr() as *const u32,
    };

    unsafe {
        device
            .create_shader_module(&shader_module_create_info, None)
            .expect("Failed to create shader module")
    }
}

fn create_vertex_input_state_create_info(
    attribute_descriptions: &[VertexInputAttributeDescription],
    binding_description: &[VertexInputBindingDescription],
) -> PipelineVertexInputStateCreateInfo {
    PipelineVertexInputStateCreateInfo {
        s_type: StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: PipelineVertexInputStateCreateFlags::empty(),
        vertex_attribute_description_count: attribute_descriptions.len() as u32,
        p_vertex_attribute_descriptions: attribute_descriptions.as_ptr(),
        vertex_binding_description_count: binding_description.len() as u32,
        p_vertex_binding_descriptions: binding_description.as_ptr(),
    }
}

fn create_vertex_input_assembly_state_info() -> PipelineInputAssemblyStateCreateInfo {
    PipelineInputAssemblyStateCreateInfo {
        s_type: StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        flags: PipelineInputAssemblyStateCreateFlags::empty(),
        p_next: std::ptr::null(),
        primitive_restart_enable: FALSE,
        topology: PrimitiveTopology::TRIANGLE_LIST,
    }
}

fn create_viewport(swapchain_extent: &Extent2D) -> [Viewport; 1] {
    [Viewport {
        x: 0.0,
        y: 0.0,
        width: swapchain_extent.width as f32,
        height: swapchain_extent.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    }]
}

fn create_scissors(swapchain_extent: &Extent2D) -> [Rect2D; 1] {
    [Rect2D {
        offset: Offset2D { x: 0, y: 0 },
        extent: *swapchain_extent,
    }]
}

fn create_viewport_state_create_info(
    viewports: &[Viewport],
    scissors: &[Rect2D],
) -> PipelineViewportStateCreateInfo {
    PipelineViewportStateCreateInfo {
        s_type: StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: PipelineViewportStateCreateFlags::empty(),
        scissor_count: scissors.len() as u32,
        p_scissors: scissors.as_ptr(),
        viewport_count: viewports.len() as u32,
        p_viewports: viewports.as_ptr(),
    }
}

fn create_rasterization_state_create_info() -> PipelineRasterizationStateCreateInfo {
    PipelineRasterizationStateCreateInfo {
        s_type: StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: PipelineRasterizationStateCreateFlags::empty(),
        depth_clamp_enable: FALSE,
        cull_mode: CullModeFlags::BACK,
        front_face: FrontFace::CLOCKWISE,
        line_width: 1.0,
        polygon_mode: PolygonMode::FILL,
        rasterizer_discard_enable: FALSE,
        depth_bias_clamp: 0.0,
        depth_bias_constant_factor: 0.0,
        depth_bias_enable: FALSE,
        depth_bias_slope_factor: 0.0,
    }
}

fn create_multisample_state_create_info() -> PipelineMultisampleStateCreateInfo {
    PipelineMultisampleStateCreateInfo {
        s_type: StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        flags: PipelineMultisampleStateCreateFlags::empty(),
        p_next: std::ptr::null(),
        rasterization_samples: SampleCountFlags::TYPE_1,
        sample_shading_enable: FALSE,
        min_sample_shading: 0.0,
        p_sample_mask: std::ptr::null(),
        alpha_to_one_enable: FALSE,
        alpha_to_coverage_enable: FALSE,
    }
}

fn create_stencil_state() -> StencilOpState {
    StencilOpState {
        fail_op: StencilOp::KEEP,
        pass_op: StencilOp::KEEP,
        depth_fail_op: StencilOp::KEEP,
        compare_op: CompareOp::ALWAYS,
        compare_mask: 0,
        write_mask: 0,
        reference: 0,
    }
}

fn create_depth_state_create_info(
    stencil_state: &StencilOpState,
) -> PipelineDepthStencilStateCreateInfo {
    PipelineDepthStencilStateCreateInfo {
        s_type: StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: PipelineDepthStencilStateCreateFlags::empty(),
        depth_test_enable: FALSE,
        depth_write_enable: FALSE,
        depth_compare_op: CompareOp::LESS_OR_EQUAL,
        depth_bounds_test_enable: FALSE,
        stencil_test_enable: FALSE,
        front: *stencil_state,
        back: *stencil_state,
        max_depth_bounds: 1.0,
        min_depth_bounds: 0.0,
    }
}

fn create_color_blend_attachment_states() -> [PipelineColorBlendAttachmentState; 1] {
    [PipelineColorBlendAttachmentState {
        blend_enable: FALSE,
        color_write_mask: ColorComponentFlags::all(),
        src_color_blend_factor: BlendFactor::ONE,
        dst_color_blend_factor: BlendFactor::ZERO,
        color_blend_op: BlendOp::ADD,
        src_alpha_blend_factor: BlendFactor::ONE,
        dst_alpha_blend_factor: BlendFactor::ZERO,
        alpha_blend_op: BlendOp::ADD,
    }]
}

fn create_color_blend_state(
    color_blend_attachment_states: &[PipelineColorBlendAttachmentState; 1],
) -> PipelineColorBlendStateCreateInfo {
    PipelineColorBlendStateCreateInfo {
        s_type: StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: PipelineColorBlendStateCreateFlags::empty(),
        logic_op_enable: FALSE,
        logic_op: LogicOp::COPY,
        attachment_count: color_blend_attachment_states.len() as u32,
        p_attachments: color_blend_attachment_states.as_ptr(),
        blend_constants: [0.0, 0.0, 0.0, 0.0],
    }
}

fn create_pipeline_layout_create_info() -> PipelineLayoutCreateInfo {
    PipelineLayoutCreateInfo {
        s_type: StructureType::PIPELINE_LAYOUT_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: PipelineLayoutCreateFlags::empty(),
        set_layout_count: 0,
        p_set_layouts: std::ptr::null(),
        push_constant_range_count: 0,
        p_push_constant_ranges: std::ptr::null(),
    }
}
