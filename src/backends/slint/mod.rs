//! Support for drawing blink bar using 'slint' as the GUI backend.
use crate::config::WindowSize;

mod winit_helper;

slint::include_modules!();

pub fn render(config: &crate::config::Config, logic: impl Fn(String) + 'static) -> Result<(), Box<dyn std::error::Error>> {
    let app = AppWindow::new()?;

    // Position the window
    let monitor_dimensions = winit_helper::position_window(app.window(), &config.size, &config.location);

    // Fixup the window dimensions
    match (&config.size, monitor_dimensions) {
        (WindowSize::Percent(x, y), Some(dims)) => {
            app.set_window_height((dims.height as f32 * y) as i32);
            app.set_window_width((dims.width as f32 * x) as i32);
        },
        (WindowSize::Pixels(x, y), _) => {
            app.set_window_height(*y as i32);
            app.set_window_width(*x as i32);
        },
        (WindowSize::Percent(_, _), None) => {
            println!("Couldn't get monitor dimensions, using default size")
        },
    };

    // Ensure the window gains focus on startup
    //
    // This doesn't work before entering the event loop
    // so we need this weird thread
    let app_handle = app.as_weak();
    std::thread::spawn(move || {
        app_handle
            .upgrade_in_event_loop(move |handle| handle.invoke_focus())
            .ok();
    });

    // Handle new inputs (core logic)
    let app_input_handle = app.as_weak();
    app.on_new_input(move |input| {
        let app = app_input_handle.unwrap();

        logic(input.to_string());

        // Task, task'd
        app.window().set_minimized(true);
    });

    app.run()?;

    Ok(())
}