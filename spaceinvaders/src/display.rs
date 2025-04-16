use sdl2::{render::Canvas, sys::Window, VideoSubsystem};
use anyhow::Result;

pub struct Display {
    
}

impl Display {
    pub fn try_new(video_subsystem: VideoSubsystem) -> Result<Self> {
        Ok(Self{})
    }
}