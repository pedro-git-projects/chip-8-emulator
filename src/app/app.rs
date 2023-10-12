use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

use super::config::WindowConfig;

pub struct App {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    is_running: bool,
}

impl App {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn new(config: WindowConfig) -> Result<Self, String> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(config.title, config.width, config.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let is_running = true;

        Ok(Self {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            is_running,
        })
    }

    pub fn process_input(&mut self) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
        Ok(())
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 0));
        self.canvas.fill_rect(Rect::new(0, 0, 40, 40));

        self.canvas.present();
    }
}
