use ash::extensions::khr::Surface;
use ash::extensions::khr::WaylandSurface;
#[cfg(feature = "xlib")]
use ash::extensions::khr::XlibSurface;
#[cfg(feature = "xlib")]
use ash::vk::{Display, Window, XlibSurfaceCreateInfoKHR};
use ash::vk::{StructureType, SurfaceKHR, WaylandSurfaceCreateInfoKHR};
use ash::{Entry, Instance};
use log::debug;
use winit::platform::wayland::WindowExtWayland;
use winit::window::Window as WinitWindow;

pub struct PotatoSurface {
    pub surface_loader: Surface,
    pub surface: SurfaceKHR,
}
//TODO make this support multiple platforms
//TODO support multiple windows
pub fn create_surface(entry: &Entry, instance: &Instance, window: &WinitWindow) -> PotatoSurface {
    let surface = unsafe { create_platform_surface(entry, instance, window) };

    let surface_loader = Surface::new(entry, instance);

    PotatoSurface {
        surface_loader,
        surface: surface.unwrap(),
    }
}

#[cfg(feature = "xlib")]
unsafe fn create_platform_surface(
    entry: &Entry,
    instance: &Instance,
    window: &WinitWindow,
) -> Result<SurfaceKHR, ash::vk::Result> {
    debug!("Creating Xlib surface");
    let x11_create_info = XlibSurfaceCreateInfoKHR {
        s_type: StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
        p_next: std::ptr::null(),
        flags: Default::default(),
        window: window.xlib_window().unwrap() as Window,
        dpy: window.xlib_display().unwrap() as *mut Display,
    };
    let xlib_surface_loader = XlibSurface::new(entry, instance);
    xlib_surface_loader.create_xlib_surface(&x11_create_info, None)
}

#[cfg(feature = "wayland")]
unsafe fn create_platform_surface(
    entry: &Entry,
    instance: &Instance,
    window: &WinitWindow,
) -> Result<SurfaceKHR, ash::vk::Result> {
    debug!("Creating Wayland surface");
    let wayland_create_info = WaylandSurfaceCreateInfoKHR {
        s_type: StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
        p_next: std::ptr::null(),
        flags: Default::default(),
        display: window.wayland_display().unwrap(),
        surface: window.wayland_surface().unwrap(),
    };
    let wayland_surface_loader = WaylandSurface::new(entry, instance);
    wayland_surface_loader.create_wayland_surface(&wayland_create_info, None)
}
