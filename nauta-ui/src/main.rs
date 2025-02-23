use std::{num::NonZeroU32, rc::Rc};

use softbuffer::{Context, Surface};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const WIDTH: NonZeroU32 = NonZeroU32::new(800).unwrap();
const HEIGHT: NonZeroU32 = NonZeroU32::new(600).unwrap();

#[derive(Default)]
struct App {
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(CARGO_PKG_NAME)
            .with_inner_size(PhysicalSize::new(WIDTH.get(), HEIGHT.get()));

        let Ok(window) = event_loop
            .create_window(window_attributes)
            .map(Rc::new)
            .inspect_err(|err| {
                tracing::error!(error = err.to_string());
            })
        else {
            return;
        };

        let Ok(context) = Context::new(window.clone()).inspect_err(|err| {
            tracing::error!(error = err.to_string());
        }) else {
            return;
        };

        self.surface = Surface::new(&context, window.clone())
            .inspect_err(|err| {
                tracing::error!(error = err.to_string());
            })
            .ok();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                let (Some(surface), Some(width), Some(height)) = (
                    self.surface.as_mut(),
                    NonZeroU32::new(size.width),
                    NonZeroU32::new(size.height),
                ) else {
                    return;
                };

                if let Err(err) = surface.resize(width, height) {
                    tracing::error!(error = err.to_string());
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(surface) = self.surface.as_mut() else {
                    return;
                };

                let Ok(buffer) = surface.buffer_mut().inspect_err(|err| {
                    tracing::error!(error = err.to_string());
                }) else {
                    return;
                };

                if let Err(err) = buffer.present() {
                    tracing::error!(error = err.to_string());
                }
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {}
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {}
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    Ok(event_loop.run_app(&mut app)?)
}
