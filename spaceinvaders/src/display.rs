use sdl2::{VideoSubsystem, render::Canvas, sys::Window};

use crate::errors::{Error, Result};

const PIXEL_WIDTH: usize = 224;
const PIXEL_HEIGHT: usize = 256;
pub const PIXEL_BYTES: usize = PIXEL_WIDTH * PIXEL_HEIGHT / 8;

const DEFAULT_WINDOW_SCALE: usize = 15;
const DEFAULT_WINDOW_WIDTH: usize = PIXEL_WIDTH * DEFAULT_WINDOW_SCALE;
const DEFAULT_WINDOW_HEIGHT: usize = PIXEL_HEIGHT * DEFAULT_WINDOW_SCALE;

pub struct Display {
    canvas: Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn try_new(video_subsystem: VideoSubsystem) -> Result<Self> {
        let window: sdl2::video::Window = video_subsystem
            .window(
                "Space Invaders Emulator",
                DEFAULT_WINDOW_WIDTH as u32,
                DEFAULT_WINDOW_HEIGHT as u32,
            )
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .map_err(|e| Error::Sdl(e.to_string()))?;

        let mut canvas: Canvas<sdl2::video::Window> = window
            .into_canvas()
            .build()
            .map_err(|e| Error::Sdl(e.to_string()))?;

        canvas.clear();
        canvas.present();

        Ok(Self { canvas })
    }

    pub fn render(&mut self, vram: &[u8]) -> Result<()> {
        todo!()
    }
}
