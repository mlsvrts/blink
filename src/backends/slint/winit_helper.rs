/// Helper for centering a window using winit
///
/// Credit prof79
use crate::config::{WindowSize, WindowPosition};

use i_slint_backend_winit::WinitWindowAccessor;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    monitor::MonitorHandle,
    window::Window
};

/// Centers the window on the primary monitor
///
/// Returns the physical size of the monitor
pub fn position_window(
    window: &slint::Window,
    size: &WindowSize,
    location: &WindowPosition,
) -> Option<slint::PhysicalSize> {
    window.with_winit_window(|window| {
        if let Some(monitor) = window.primary_monitor() {
            set_location(window, monitor, size, location)
        } else {
            // Guess :)
            slint::PhysicalSize {
                width: 1920,
                height: 1080,
            }
        }
    })
}

/// Sets the window to be centered on the monitor
/// 
/// Does not work on all platforms (Android, Wayland, iOS)
fn set_location(
    window: &Window,
    monitor: MonitorHandle,
    size: &WindowSize,
    location: &WindowPosition,
) -> slint::PhysicalSize {
    let monitor_size = monitor.size();
    let monitor_positions = monitor.position();

    let window_size = PhysicalSize {
        width: match size {
            WindowSize::Percent(w, _) => (monitor_size.width as f32 * w) as u32,
            WindowSize::Pixels(w, _) => *w,
        },
        height: match size {
            WindowSize::Percent(_, h) => (monitor_size.height as f32 * h) as u32,
            WindowSize::Pixels(_, h) => *h,
        },
    };

    let window_position = PhysicalPosition {
        x: match location {
            WindowSize::Percent(x, _) => (monitor_positions.x as f32 + (monitor_size.width as f32 * x) - (window_size.width as f32 * x)) as i32,
            WindowSize::Pixels(x, _) => *x as i32,
        },
        y: match location {
            WindowSize::Percent(_, y) => (monitor_positions.y as f32 + (monitor_size.height as f32 * y) - (window_size.height as f32 * y)) as i32,
            WindowSize::Pixels(_, y) => *y as i32,
        },
    };

    window.set_outer_position(window_position);

    slint::PhysicalSize {
        width: monitor_size.width,
        height: monitor_size.height,
    }
}