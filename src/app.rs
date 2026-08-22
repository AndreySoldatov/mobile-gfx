use crate::{runtime::Runtime, UserState};
use glam::Vec2;
use winit::{
    application::ApplicationHandler,
    event::{MouseButton, WindowEvent},
    event_loop::ControlFlow,
};

pub struct App<U: UserState> {
    runtime: Option<Runtime<U>>,
    cursor_pos: Vec2,
}

impl<U: UserState> App<U> {
    pub fn new() -> Self {
        Self {
            runtime: None,
            cursor_pos: Vec2::ZERO,
        }
    }
}

impl<U: UserState> ApplicationHandler for App<U> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &mut self.runtime {
            Some(runtime) => {
                if runtime.surface.is_none() {
                    runtime.create_surface();
                }
            }
            None => {
                event_loop.set_control_flow(ControlFlow::Poll);
                self.runtime = Some(Runtime::new(event_loop));
            }
        }
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.runtime.as_mut().unwrap().surface = None
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                if let Some(runtime) = &mut self.runtime {
                    runtime.update();
                }
            }
            WindowEvent::Destroyed | WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }
                if let Some(runtime) = &mut self.runtime {
                    runtime.resize(size);
                }
            }
            WindowEvent::Touch(touch) => {
                if let Some(runtime) = &mut self.runtime {
                    runtime.input.touch_event(touch);
                }
            }
            WindowEvent::MouseInput {
                device_id: _d,
                state,
                button,
            } => {
                if button != MouseButton::Left {
                    return;
                }
                if let Some(runtime) = &mut self.runtime {
                    if button != MouseButton::Left {
                        return;
                    }
                    runtime.input.mouse_event(self.cursor_pos, state);
                }
            }
            WindowEvent::CursorMoved {
                device_id: _d,
                position,
            } => {
                self.cursor_pos = Vec2::new(position.x as f32, position.y as f32);
                if let Some(runtime) = &mut self.runtime {
                    runtime.input.mouse_moved(self.cursor_pos);
                }
            }
            _ => {}
        }
    }
}
