use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;

use crate::errors::Result;

const UFO_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/ufo_highpitch.wav"));
const SHOOT_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/shoot.wav"));
const PLAYER_DEATH_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/explosion.wav"));
const INVADER_DEATH_SOUND: Cursor<&[u8]> =
    Cursor::new(include_bytes!("../sounds/invaderkilled.wav"));
const INVADER_1_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/fastinvader1.wav"));
const INVADER_2_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/fastinvader2.wav"));
const INVADER_3_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/fastinvader3.wav"));
const INVADER_4_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/fastinvader4.wav"));
const INVADER_HIT_SOUND: Cursor<&[u8]> = Cursor::new(include_bytes!("../sounds/ufo_lowpitch.wav"));

pub struct Audio {
    _stream: OutputStream,
    sinks: Sinks,

    // Sound enabled
    ufo_enabled: bool,
    shoot_enabled: bool,
    player_death_enabled: bool,
    invader_death_enabled: bool,
    invader1_enabled: bool,
    invader2_enabled: bool,
    invader3_enabled: bool,
    invader4_enabled: bool,
    invader_hit_enabled: bool,
}

struct Sinks {
    ufo: Sink,
    shoot: Sink,
    player_death: Sink,
    invader_death: Sink,
    invader1: Sink,
    invader2: Sink,
    invader3: Sink,
    invader4: Sink,
    invader_hit: Sink,
}

impl Audio {
    pub fn try_new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;

        let sinks = Sinks {
            ufo: Sink::try_new(&stream_handle)?,
            shoot: Sink::try_new(&stream_handle)?,
            player_death: Sink::try_new(&stream_handle)?,
            invader_death: Sink::try_new(&stream_handle)?,
            invader1: Sink::try_new(&stream_handle)?,
            invader2: Sink::try_new(&stream_handle)?,
            invader3: Sink::try_new(&stream_handle)?,
            invader4: Sink::try_new(&stream_handle)?,
            invader_hit: Sink::try_new(&stream_handle)?,
        };

        Ok(Self {
            _stream,
            sinks,

            ufo_enabled: false,
            shoot_enabled: false,
            player_death_enabled: false,
            invader_death_enabled: false,
            invader1_enabled: false,
            invader2_enabled: false,
            invader3_enabled: false,
            invader4_enabled: false,
            invader_hit_enabled: false,
        })
    }

    pub fn play_port3(&mut self, bit: u8) -> Result<()> {
        // UFO (looping)
        play_looping(
            bit & 1 != 0,
            &mut self.ufo_enabled,
            &mut self.sinks.ufo,
            UFO_SOUND,
        )?;

        // Shoot
        play_once(
            (bit >> 1) & 1 != 0,
            &mut self.shoot_enabled,
            &mut self.sinks.shoot,
            SHOOT_SOUND,
        )?;

        // Player death
        play_once(
            (bit >> 2) & 1 != 0,
            &mut self.player_death_enabled,
            &mut self.sinks.player_death,
            PLAYER_DEATH_SOUND,
        )?;

        // Invader death
        play_once(
            (bit >> 3) & 1 != 0,
            &mut self.invader_death_enabled,
            &mut self.sinks.invader_death,
            INVADER_DEATH_SOUND,
        )?;

        Ok(())
    }

    pub fn play_port5(&mut self, bit: u8) -> Result<()> {
        // Invader 1
        play_once(
            bit & 1 != 0,
            &mut self.invader1_enabled,
            &mut self.sinks.invader1,
            INVADER_1_SOUND,
        )?;

        // Invader 2
        play_once(
            (bit >> 1) & 1 != 0,
            &mut self.invader2_enabled,
            &mut self.sinks.invader2,
            INVADER_2_SOUND,
        )?;

        // Invader 3
        play_once(
            (bit >> 2) & 1 != 0,
            &mut self.invader3_enabled,
            &mut self.sinks.invader3,
            INVADER_3_SOUND,
        )?;

        // Invader 4
        play_once(
            (bit >> 3) & 1 != 0,
            &mut self.invader4_enabled,
            &mut self.sinks.invader4,
            INVADER_4_SOUND,
        )?;

        // UFO hit
        play_once(
            (bit >> 4) & 1 != 0,
            &mut self.invader_hit_enabled,
            &mut self.sinks.invader_hit,
            INVADER_HIT_SOUND,
        )?;

        Ok(())
    }
}

fn play_looping(
    pin: bool,
    enabled: &mut bool,
    sink: &mut Sink,
    sound: Cursor<&'static [u8]>,
) -> Result<()> {
    if pin && !*enabled {
        sink.append(Decoder::new_wav(sound)?.repeat_infinite());
        *enabled = true;
    } else if !pin && *enabled {
        sink.stop();
        *enabled = false;
    }

    Ok(())
}

fn play_once(
    pin: bool,
    enabled: &mut bool,
    sink: &mut Sink,
    sound: Cursor<&'static [u8]>,
) -> Result<()> {
    if pin && !*enabled {
        sink.append(Decoder::new_wav(sound)?);
        *enabled = true;
    } else if !pin && *enabled {
        *enabled = false;
    }

    Ok(())
}
