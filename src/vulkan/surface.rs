use ash::extensions::khr::Surface;
use ash::extensions::khr::XlibSurface;
use ash::vk::{Display, StructureType, SurfaceKHR, Window, XlibSurfaceCreateInfoKHR};
use ash::{Entry, Instance};
use winit::platform::unix::WindowExtUnix;
use winit::window::Window as WinitWindow;

pub struct PotatoSurface {
    pub surface_loader: Surface,
    pub surface: SurfaceKHR,
}
//TODO make this support multiple platforms
//TODO support multiple windows
pub fn create_surface(entry: &Entry, instance: &Instance, window: &WinitWindow) -> PotatoSurface {
    let surface = unsafe {
        let x11_create_info = XlibSurfaceCreateInfoKHR {
            s_type: StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: window.xlib_window().unwrap() as Window,
            dpy: window.xlib_display().unwrap() as *mut Display,
        };
        let xlib_surface_loader = XlibSurface::new(entry, instance);
        xlib_surface_loader.create_xlib_surface(&x11_create_info, None)
    };

    let surface_loader = Surface::new(entry, instance);

    PotatoSurface {
        surface_loader,
        surface: surface.unwrap(),
    }
}
