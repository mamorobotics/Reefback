use glium::glutin::surface::WindowSurface;
use glium::{Display, Surface};
use imgui::{Context, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::winit::dpi::LogicalSize;
use imgui_winit_support::winit::event::{Event, WindowEvent};
use imgui_winit_support::winit::event_loop::EventLoop;
use imgui_winit_support::winit::window::WindowAttributes;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;

#[allow(dead_code)] // annoyingly, RA yells that this is unusued
pub fn simple_init<F: FnMut(&mut bool, &mut Ui) + 'static>(title: &str, run_ui: F) {
    init_with_startup(title, |_, _, _| {}, run_ui);
}

pub fn init_with_startup<FInit, FUi>(title: &str, mut startup: FInit, mut run_ui: FUi)
where
    FInit: FnMut(&mut Context, &mut Renderer, &Display<WindowSurface>) + 'static,
    FUi: FnMut(&mut bool, &mut Ui) + 'static,
{
    let mut imgui = Context::create();

    let title = match Path::new(&title).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => title,
    };
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let window_attributes = WindowAttributes::default()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1024, 768));
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);
    let mut renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut platform = WinitPlatform::new(&mut imgui);
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    let mut last_frame = Instant::now();

    startup(&mut imgui, &mut renderer, &display);

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let ui = imgui.frame();

                let mut run = true;
                run_ui(&mut run, ui);
                if !run {
                    window_target.exit();
                }

                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(ui, &window);
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                if new_size.width > 0 && new_size.height > 0 {
                    display.resize((new_size.width, new_size.height));
                }
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            event => {
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
        })
        .expect("EventLoop error");
}