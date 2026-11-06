use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use rodio::{OutputStreamBuilder, Sink, Source, source::SineWave};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{
    cpu::cpu::SimCPU,
    graphic::screen::{SCREEN_HEIGHT, SCREEN_WIDTH, Screen},
    input::io::IO,
    memory::memory::Memory,
    sound::sound::Sound,
    stack::stack::Stack,
    timer::timer::Timer,
};

const WINDOW_SCALE: u32 = 20;
const CPU_FREQUENCY: u64 = 700;
const CPU_CYCLE: Duration = Duration::from_nanos(1_000_000_000 / CPU_FREQUENCY);
const FRAME_DURATION: Duration = Duration::from_millis(16);

pub struct Display {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    cpu: SimCPU,
    memory: Memory,
    stack: Stack,
    screen: Screen,
    io: IO,
    timer: Timer,
    last_timer_update: Instant,
    last_frame_update: Instant,
    cpu_cycle_accumulator: Duration,
    sound: Sound,
    audio_sink: Sink,
}

impl Display {
    pub fn run(cpu: SimCPU, memory: Memory, stack: Stack, io: IO, timer: Timer, sound: Sound) {
        let audio_stream =
            OutputStreamBuilder::open_default_stream().expect("Could not open audio output");
        let audio_sink = Sink::connect_new(audio_stream.mixer());
        audio_sink.append(SineWave::new(440.0).amplify(0.15).repeat_infinite());
        audio_sink.pause();

        let now = Instant::now();
        let mut app = Self {
            window: None,
            pixels: None,
            cpu,
            memory,
            stack,
            screen: Screen::default(),
            io,
            timer,
            last_timer_update: now,
            last_frame_update: now,
            cpu_cycle_accumulator: Duration::ZERO,
            sound,
            audio_sink,
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
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    self.io
                        .set_key_pressed(key, event.state == ElementState::Pressed);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_timer_update);
        self.timer.advance(elapsed);
        self.sound.advance(elapsed);
        self.last_timer_update = now;

        self.cpu_cycle_accumulator += elapsed;
        while self.cpu_cycle_accumulator >= CPU_CYCLE {
            self.cpu.run_next_op(
                &mut self.stack,
                &mut self.memory,
                &mut self.screen,
                &mut self.io,
                &mut self.timer,
                &mut self.sound,
            );
            self.cpu_cycle_accumulator -= CPU_CYCLE;
        }

        if self.sound.is_active() {
            self.audio_sink.play();
        } else {
            self.audio_sink.pause();
        }

        if now.duration_since(self.last_frame_update) >= FRAME_DURATION {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
            self.last_frame_update = now;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + CPU_CYCLE.saturating_sub(self.cpu_cycle_accumulator),
        ));
    }
}
