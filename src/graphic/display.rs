use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::{
    cpu::cpu::SimCPU,
    graphic::screen::{SCREEN_HEIGHT, SCREEN_WIDTH, Screen},
    memory::memory::Memory,
    stack::stack::Stack,
};

const WINDOW_SCALE: u32 = 10;
const INSTRUCTIONS_PER_FRAME: usize = 10;
const FRAME_DURATION: Duration = Duration::from_millis(16);

pub struct Display {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    cpu: SimCPU,
    memory: Memory,
    stack: Stack,
    screen: Screen,
}

impl Display {
    pub fn run(cpu: SimCPU, memory: Memory, stack: Stack) {
        let mut app = Self {
            window: None,
            pixels: None,
            cpu,
            memory,
            stack,
            screen: Screen::default(),
        };
        let event_loop = EventLoop::new().expect("Could not create event loop");

        event_loop.run_app(&mut app).expect("Error running app");
    }
}

impl ApplicationHandler for Display {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("CHIP-8 Emulator")
                        .with_inner_size(PhysicalSize::new(
                            (SCREEN_WIDTH as u32) * WINDOW_SCALE,
                            (SCREEN_HEIGHT as u32) * WINDOW_SCALE,
                        ))
                        .with_visible(true),
                )
                .expect("Could not create window"),
        );
        let surface = SurfaceTexture::new(
            (SCREEN_WIDTH as u32) * WINDOW_SCALE,
            (SCREEN_HEIGHT as u32) * WINDOW_SCALE,
            window.clone(),
        );
        let pixels = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface)
            .expect("Could not create pixel surface");

        window.request_redraw();
        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.window.as_ref().map(|window| window.id()) != Some(window_id) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.pixels
                    .as_mut()
                    .expect("Pixel surface is not initialized")
                    .resize_surface(size.width, size.height)
                    .expect("Could not resize pixel surface");
            }
            WindowEvent::RedrawRequested => {
                let frame = self
                    .pixels
                    .as_mut()
                    .expect("Pixel surface is not initialized")
                    .frame_mut();

                for (rgba, &on) in frame.chunks_exact_mut(4).zip(self.screen.pixels().iter()) {
                    rgba.copy_from_slice(if on {
                        &[0xFF, 0xFF, 0xFF, 0xFF]
                    } else {
                        &[0x00, 0x00, 0x00, 0xFF]
                    });
                }

                self.pixels
                    .as_ref()
                    .expect("Pixel surface is not initialized")
                    .render()
                    .expect("Could not render frame");
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        for _ in 0..INSTRUCTIONS_PER_FRAME {
            self.cpu
                .run_next_op(&mut self.stack, &mut self.memory, &mut self.screen);
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + FRAME_DURATION));
    }
}
