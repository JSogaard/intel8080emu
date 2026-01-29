use sdl2::{VideoSubsystem, pixels::Color, rect::Rect, render::Canvas, sys::Window};

use crate::errors::{Error, Result};

const PIXEL_WIDTH: u32 = 224;
const PIXEL_HEIGHT: u32 = 256;
pub const PIXEL_BYTES: u32 = PIXEL_WIDTH * PIXEL_HEIGHT / 8;

const PIXEL_SIZE: u32 = 15;
const WINDOW_WIDTH: u32 = PIXEL_WIDTH * PIXEL_SIZE;
const WINDOW_HEIGHT: u32 = PIXEL_HEIGHT * PIXEL_SIZE;

const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const FOREGROUND_COLOR: Color = Color::RGB(245, 245, 245);

pub struct Display {
    canvas: Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn try_new(video_subsystem: VideoSubsystem) -> Result<Self> {
        let window: sdl2::video::Window = video_subsystem
            .window("Space Invaders Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
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
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();

        self.canvas.set_draw_color(FOREGROUND_COLOR);

        for x in 0..224 {
            for y in (0..256).rev() {
                let pixel_num: usize = x * 256 + 255 - y;
                let byte = pixel_num / 8;
                let bit = pixel_num % 8;
                let pixel_on = ((vram[byte] >> bit) & 0x1) != 0;

                if pixel_on {
                    let rect = Rect::new(
                        x as i32 * PIXEL_SIZE,
                        y as i32 * PIXEL_SIZE,
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                    );
                    self.canvas.fill_rect(rect)?;
                }
            }
        }

        Ok(())
    }
}
