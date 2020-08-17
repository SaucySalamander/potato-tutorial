use super::command_pool::{create_command_buffers, create_command_pool};
use super::constants::{MAX_FRAMES_IN_FLIGHT, VALIDATION};
use super::device::create_logical_device;
use super::framebuffers::create_framebuffers;
use super::graphics_pipeline::create_graphics_pipeline;
use super::instance::create_instance;
use super::physical_device::{describe_device, select_physical_device};
use super::queue_family::QueueFamily;
use super::render_pass::create_render_pass;
use super::surface::{create_surface, PotatoSurface};
use super::swapchain::{create_swapchain, PotatoSwapChain};
use super::sync_objects::create_sync_objects;
use super::vertex::create_vertex_buffer;
use super::vulk_validation_layers::setup_debug_utils;
use ash::extensions::ext::DebugUtils;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::vk::{
    Buffer, CommandBuffer, CommandPool, DebugUtilsMessengerEXT, DeviceMemory, Fence, Framebuffer,
    PhysicalDevice, Pipeline, PipelineLayout, PipelineStageFlags, PresentInfoKHR, Queue,
    RenderPass, Result, Semaphore, StructureType, SubmitInfo,
};
use ash::Device;
use ash::Entry;
use ash::Instance;
use log::debug;
use std::collections::HashMap;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder, WindowId},
};

pub struct VulkanApiObjects {
    windows: HashMap<WindowId, Window>,
    _entry: Entry,
    instance: Instance,
    surface: PotatoSurface,
    queue_family: QueueFamily,
    debug_utils_loader: DebugUtils,
    debug_messenger: DebugUtilsMessengerEXT,
    physical_device: PhysicalDevice,
    device: Device,
    graphics_queue: Queue,
    swapchain: PotatoSwapChain,
    pipeline_layout: PipelineLayout,
    render_pass: RenderPass,
    graphics_pipeline: Pipeline,
    swapchain_framebuffers: Vec<Framebuffer>,
    command_pool: CommandPool,
    command_buffers: Vec<CommandBuffer>,
    image_available_semaphores: Vec<Semaphore>,
    render_finished_semaphores: Vec<Semaphore>,
    in_flight_fences: Vec<Fence>,
    current_frame: usize,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
}

impl VulkanApiObjects {
    pub fn init(event_loop: &EventLoop<()>) -> VulkanApiObjects {
        debug!("Init window");
        let window = VulkanApiObjects::init_window(&event_loop, "origin");
        debug!("Init entry");
        let entry = Entry::new().unwrap();
        debug!("Init instance");
        let instance = create_instance(&entry);
        debug!("Init debug utils");
        let (debug_utils_loader, debug_messenger) = setup_debug_utils(&entry, &instance);
        debug!("Init surface");
        let potato_surface = create_surface(&entry, &instance, &window);
        debug!("Init physical device");
        let physical_device = select_physical_device(&instance, &potato_surface);

        describe_device(&instance, physical_device);

        debug!("Init logical device");
        let (logical_device, queue_family) =
            create_logical_device(&instance, physical_device, &potato_surface);
        debug!("Init swapchain");
        let swapchain = create_swapchain(
            &instance,
            &logical_device,
            physical_device,
            &potato_surface,
            &queue_family,
        );
        debug!("Init graphics queue");
        let graphics_queue = unsafe {
            logical_device.get_device_queue(queue_family.graphics_family.unwrap() as u32, 0)
        };
        debug!("Init render pass");
        let render_pass = create_render_pass(&logical_device, swapchain.swapchain_format);
        debug!("Init graphics pipeline");
        let (graphics_pipeline, pipeline_layout) =
            create_graphics_pipeline(&logical_device, render_pass, swapchain.swapchain_extent);
        debug!("Init framebuffers");
        let swapchain_framebuffers = create_framebuffers(
            &logical_device,
            render_pass,
            &swapchain.swapchain_image_views,
            &swapchain.swapchain_extent,
        );
        debug!("Init command pool");
        let command_pool = create_command_pool(&logical_device, &queue_family);
        debug!("Init vertex buffer");
        let (vertex_buffer, vertex_buffer_memory) =
            create_vertex_buffer(&instance, &logical_device, physical_device, command_pool, graphics_queue);
        debug!("Init command buffers");
        let command_buffers = create_command_buffers(
            &logical_device,
            command_pool,
            graphics_pipeline,
            &swapchain_framebuffers,
            render_pass,
            swapchain.swapchain_extent,
            vertex_buffer,
        );
        debug!("Init sync objects");
        let sync_objects = create_sync_objects(&logical_device);

        let mut windows = HashMap::new();
        windows.insert(window.id(), window);

        VulkanApiObjects {
            windows,
            _entry: entry,
            instance,
            surface: potato_surface,
            queue_family,
            debug_utils_loader,
            debug_messenger,
            physical_device,
            device: logical_device,
            graphics_queue,
            swapchain,
            pipeline_layout,
            render_pass,
            graphics_pipeline,
            swapchain_framebuffers,
            command_pool,
            command_buffers,
            image_available_semaphores: sync_objects.image_available_semaphores,
            render_finished_semaphores: sync_objects.render_finished_semaphores,
            in_flight_fences: sync_objects.inflight_fences,
            current_frame: 0,
            vertex_buffer,
            vertex_buffer_memory,
        }
    }

    pub fn draw(&mut self) {
        let wait_fences = [self.in_flight_fences[self.current_frame]];
        let (image_index, _is_sub_optimal) = unsafe {
            self.device
                .wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failed to wait for Fence!");

            let result = self.swapchain.swapchain_loader.acquire_next_image(
                self.swapchain.swapchain,
                std::u64::MAX,
                self.image_available_semaphores[self.current_frame],
                Fence::null(),
            );
            match result {
                Ok(image_index) => image_index,
                Err(vk_result) => match vk_result {
                    Result::ERROR_OUT_OF_DATE_KHR => {
                        self.recreate_swapchain();
                        return;
                    }
                    _ => panic!("Failed to acquire swap chain image"),
                },
            }
        };

        let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
        let wait_stages = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];

        let submit_infos = [SubmitInfo {
            s_type: StructureType::SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_buffers[image_index as usize],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        }];
        unsafe {
            self.device
                .reset_fences(&wait_fences)
                .expect("Failed to reset Fence!");

            self.device
                .queue_submit(
                    self.graphics_queue,
                    &submit_infos,
                    self.in_flight_fences[self.current_frame],
                )
                .expect("Failed to execute queue submit.");
        }

        let swapchains = [self.swapchain.swapchain];

        let present_info = PresentInfoKHR {
            s_type: StructureType::PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: std::ptr::null_mut(),
        };

        let result = unsafe {
            self.swapchain
                .swapchain_loader
                .queue_present(self.graphics_queue, &present_info)
        };

        let is_resized = match result {
            Ok(_) => false,
            Err(vk_result) => match vk_result {
                Result::ERROR_OUT_OF_DATE_KHR | Result::SUBOPTIMAL_KHR => true,
                _ => panic!("Failed to execute queue present"),
            },
        };

        if is_resized {
            self.recreate_swapchain();
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    fn recreate_swapchain(&mut self) {
        unsafe {
            self.device
                .device_wait_idle()
                .expect("Failed to wait on device")
        };
        self.cleanup_swapchain();

        self.swapchain = create_swapchain(
            &self.instance,
            &self.device,
            self.physical_device,
            &self.surface,
            &self.queue_family,
        );
        self.render_pass = create_render_pass(&self.device, self.swapchain.swapchain_format);
        let (graphics_pipeline, pipeline_layout) = create_graphics_pipeline(
            &self.device,
            self.render_pass,
            self.swapchain.swapchain_extent,
        );
        self.graphics_pipeline = graphics_pipeline;
        self.pipeline_layout = pipeline_layout;
        self.swapchain_framebuffers = create_framebuffers(
            &self.device,
            self.render_pass,
            &self.swapchain.swapchain_image_views,
            &self.swapchain.swapchain_extent,
        );
        self.command_buffers = create_command_buffers(
            &self.device,
            self.command_pool,
            graphics_pipeline,
            &self.swapchain_framebuffers,
            self.render_pass,
            self.swapchain.swapchain_extent,
            self.vertex_buffer,
        );
    }

    fn cleanup_swapchain(&self) {
        unsafe {
            self.device
                .free_command_buffers(self.command_pool, &self.command_buffers);
            self.swapchain_framebuffers
                .iter()
                .for_each(|x| self.device.destroy_framebuffer(*x, None));
            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);
            self.swapchain
                .swapchain_image_views
                .iter()
                .for_each(|x| self.device.destroy_image_view(*x, None));
            self.swapchain
                .swapchain_loader
                .destroy_swapchain(self.swapchain.swapchain, None);
        }
    }

    fn init_window(event_loop: &EventLoopWindowTarget<()>, name: &str) -> Window {
        WindowBuilder::new()
            .with_title(name)
            .with_inner_size(LogicalSize::new(800, 600))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    pub fn init_event_loop(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, event_loop, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, window_id } => {
                    if let WindowEvent::CloseRequested = event {
                        println!("Window {:?} has received the signal to close", window_id);
                        self.windows.remove(&window_id);
                        if self.windows.is_empty() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    if let WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        is_synthetic,
                        ..
                    } = event
                    {
                        //TODO abstract keyboard input logic
                        if state == ElementState::Released
                            && virtual_keycode == Some(VirtualKeyCode::N)
                            && !is_synthetic
                        {
                            let window = VulkanApiObjects::init_window(event_loop, "spawn");
                            self.windows.insert(window.id(), window);
                        }
                    }
                }
                Event::MainEventsCleared => {
                    for (.., window) in self.windows.iter() {
                        window.request_redraw();
                    }
                }
                Event::RedrawRequested(_window_id) => {
                    self.draw();
                }
                Event::LoopDestroyed => {
                    unsafe {
                        self.device
                            .device_wait_idle()
                            .expect("Failed to wait device idle!")
                    };
                }
                _ => (),
            }
        })
    }
}

impl Drop for VulkanApiObjects {
    fn drop(&mut self) {
        unsafe {
            for i in 0..MAX_FRAMES_IN_FLIGHT {
                self.device
                    .destroy_semaphore(self.image_available_semaphores[i], None);
                self.device
                    .destroy_semaphore(self.render_finished_semaphores[i], None);
                self.device.destroy_fence(self.in_flight_fences[i], None);
            }
            self.cleanup_swapchain();
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.surface
                .surface_loader
                .destroy_surface(self.surface.surface, None);
            if VALIDATION.is_enable {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
