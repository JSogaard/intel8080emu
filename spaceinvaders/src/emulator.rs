use anyhow::Result;
use std::{fs, path::PathBuf, time::Duration};

use intel8080_core::processor::Processor;
use sdl2::{EventPump, Sdl, VideoSubsystem};

use crate::{display::Display, error::Error};

const CLOCK_SPEED: u32 = 2000000;
const FRAME_RATE: u32 = 60;
const CYCLES_PER_HALF_FRAME: u32 = CLOCK_SPEED / FRAME_RATE / 2;
const RAM_SIZE: usize = 16384;

pub struct Emulator {
    processor: Processor,
    _sdl_context: Sdl,
    eventpump: EventPump,
}

impl Emulator {
    pub fn try_new(rom_path: PathBuf, window_scale: u32) -> Result<Self> {
        let _sdl_context: Sdl = sdl2::init().map_err(|e| Error::SdlError(e.to_string()))?;
        let video_subsystem: VideoSubsystem = _sdl_context
            .video()
            .map_err(|e| Error::SdlError(e.to_string()))?;
        let eventpump = _sdl_context.event_pump().map_err(Error::SdlError)?;

        let rom: Vec<u8> = fs::read(rom_path)?;
        let processor = Processor::new(RAM_SIZE, memory_mapper);
        let display = Display::try_new(video_subsystem)?;

        Ok(Self {
            processor,
            _sdl_context,
            eventpump,
        })
    }
}

pub fn run(&mut self) -> Result<()> {
    let hframe_length = Duration::from_secs_f64(1. / FRAME_RATE as f64 / 2.);

    todo!()
}

fn memory_mapper(address: u16) -> (usize, bool) {
    todo!()
}
