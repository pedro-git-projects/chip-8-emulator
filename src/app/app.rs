use std::thread::sleep;
use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

use crate::emulator::{keyboard::map_sdl_key_to_chip8_key, Chip8};

use super::config::WindowConfig;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_WINDOW_MULTIPLIER: i32 = 10;

pub struct App {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    is_running: bool,
    chip8: Chip8,
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
        let chip = Chip8::new();

        Ok(Self {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            is_running,
            chip8: chip,
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
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(_) = map_sdl_key_to_chip8_key(keycode) {
                        self.chip8.keyboard.key_down(keycode);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(_) = map_sdl_key_to_chip8_key(keycode) {
                        self.chip8.keyboard.key_up(keycode);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn render(&mut self) {
        for x in 0..CHIP8_WIDTH {
            for y in 0..CHIP8_HEIGHT {
                if self.chip8.screen.is_set(x, y) {
                    let r = Rect::new(
                        x as i32 * CHIP8_WINDOW_MULTIPLIER,
                        y as i32 * CHIP8_WINDOW_MULTIPLIER,
                        CHIP8_WINDOW_MULTIPLIER as u32,
                        CHIP8_WINDOW_MULTIPLIER as u32,
                    );
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    self.canvas.fill_rect(r).unwrap();
                }
            }
        }
        self.canvas.present();

        // Delay timer
        if self.chip8.delay_timer > 0 {
            sleep(Duration::from_millis(1));
            self.chip8.delay_timer -= 1;
        }

        // Sound timer
        if self.chip8.sound_timer > 0 {
            sleep(Duration::from_millis(10 * self.chip8.sound_timer as u64));
            self.chip8.sound_timer = 0;
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> Result<(), String> {
        let rom_data = std::fs::read(filename).map_err(|err| err.to_string())?;
        self.chip8.load(&rom_data).map_err(|err| err.to_string())?;
        Ok(())
    }
}
