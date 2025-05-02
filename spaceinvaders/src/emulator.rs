use crate::{
    display::{Display, PIXEL_BYTES},
    errors::{Error, Result},
    io_handler::IoHandler,
};
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
    io_handler: IoHandler,
    _sdl_context: Sdl,
    eventpump: EventPump,
}

impl Emulator {
    pub fn try_new(rom_path: PathBuf, dip_settings: (u8, bool)) -> Result<Self> {
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
            io_handler: IoHandler::try_new(dip_settings)?,
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

                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        self.io_handler.set_key(keycode, true);
                    }

                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        self.io_handler.set_key(keycode, false);
                    }

                    _ => {}
                }
            }

            // Run first tick
            let mut cycles = 0;
            while cycles < CYCLES_PER_TICK {
                cycles += self.processor.execute(&mut self.io_handler)?;
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
                cycles += self.processor.execute(&mut self.io_handler)?;
            }

            let vram: &[u8] = self.processor.memory_slice(0x2400, PIXEL_BYTES)?;
            self.display.render(vram)?;

            
        }

        todo!()
    }
}

fn memory_mapper(address: u16) -> (usize, bool) {
    // Mask out the 2 unused upper RAM pins
    let address = (address & 0x3FFF) as usize;
    let is_rom = address < 0x2000;

    (address, is_rom)
}
