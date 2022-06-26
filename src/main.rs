use anyhow::{Context, Result};
use softbuffer::GraphicsContext;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    monitor::VideoMode,
    window::{Fullscreen, WindowBuilder},
};
mod camera;
mod renderer;
mod scene;
use crate::renderer::Renderer;
use crate::scene::Scene;

fn main() -> Result<()> {
    // Create the event loop
    let event_loop = EventLoop::new();

    // Create the window
    let window = WindowBuilder::new()
        .with_title("Rust Raytracer")
        .build(&event_loop)
        .context("Failed to create window.")?;

    // Create the graphics context
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    // Create renderer
    let mut renderer = Renderer::new();

    // Create the scene
    let scene = Scene::new();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Handle window events
        if let Event::WindowEvent { event, .. } = event {
            // Close event
            if event == WindowEvent::CloseRequested {
                *control_flow = ControlFlow::Exit;
            }

            // Keyboard input event
            if let WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(code),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } = event
            {
                if code == VirtualKeyCode::Escape {
                    *control_flow = ControlFlow::Exit;
                }
            }
        // Handle redraw event
        } else if let Event::RedrawRequested(_) = event {
            // Get window size
            let (width, height) = {
                let size = graphics_context.window().inner_size();
                (size.width as u16, size.height as u16)
            };

            // Render scene
            graphics_context.set_buffer(renderer.render(&scene, width, height), width, height);
        }
    });
}
