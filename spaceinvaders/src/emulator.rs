use crate::{
    display::{Display, PIXEL_BYTES},
    error::Error,
    input::Input,
};
use anyhow::Result;
use intel8080_core::processor::Processor;
use sdl2::{EventPump, Sdl, VideoSubsystem, event::Event, keyboard::Keycode};
use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

const CLOCK_SPEED: u32 = 2000000;
const FRAME_RATE: u32 = 60;
const CYCLES_PER_TICK: u32 = CLOCK_SPEED / FRAME_RATE / 2;
const TICK_LENGTH: Duration = Duration::from_nanos(1e9 as u64 / FRAME_RATE as u64 / 2);
const RAM_SIZE: usize = 16384;

pub struct Emulator {
    processor: Processor,
    display: Display,
    input: Input,
    _sdl_context: Sdl,
    eventpump: EventPump,
}

impl Emulator {
    pub fn try_new(rom_path: PathBuf) -> Result<Self> {
        let _sdl_context: Sdl = sdl2::init().map_err(|e| Error::SdlError(e.to_string()))?;
        let video_subsystem: VideoSubsystem = _sdl_context
            .video()
            .map_err(|e| Error::SdlError(e.to_string()))?;

        let eventpump = _sdl_context.event_pump().map_err(Error::SdlError)?;

        let rom: Vec<u8> = fs::read(rom_path)?;
        let mut processor = Processor::new(RAM_SIZE, memory_mapper);
        processor.load_rom(&rom, 0x0)?;
        let display = Display::try_new(video_subsystem)?;

        Ok(Self {
            processor,
            display,
            input: Input::new(),
            _sdl_context,
            eventpump,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        'main_loop: loop {
            let tick_start = Instant::now();

            for event in self.eventpump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'main_loop;
                    }
                    // TODO Handle keypresses
                    _ => {}
                }
            }

            // Run first tick
            let mut cycles = 0;
            while cycles < CYCLES_PER_TICK {
                cycles += self.processor.execute(&mut self.input)?;
            }

            // Mid-frame interrupt
            self.processor.interrupt(1)?;

            // Time padding
            let elapsed = tick_start.elapsed();
            if elapsed < TICK_LENGTH {
                std::thread::sleep(TICK_LENGTH - elapsed);
            }

            // Run second tick
            cycles = 0;
            while cycles < 2 * CYCLES_PER_TICK {
                cycles += self.processor.execute(&mut self.input)?;
            }

            let vram: &[u8] = self.processor.memory_slice(0x2400, PIXEL_BYTES)?;
            self.display.render(vram)?;

            
        }

        todo!()
    }
}

fn memory_mapper(address: u16) -> (usize, bool) {
    todo!()
}
