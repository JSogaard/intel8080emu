use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};

use crate::errors::Result;

const UFO_SOUND: &[u8] = include_bytes!("../sounds/ufo_highpitch.wav");
const SHOOT_SOUND: &[u8] = include_bytes!("../sounds/shoot.wav");
const PLAYER_DEATH_SOUND: &[u8] = include_bytes!("../sounds/explosion.wav");
const INVADER_DEATH_SOUND: &[u8] = include_bytes!("../sounds/invaderkilled.wav");
const INVADER_1_SOUND: &[u8] = include_bytes!("../sounds/fastinvader1.wav");
const INVADER_2_SOUND: &[u8] = include_bytes!("../sounds/fastinvader2.wav");
const INVADER_3_SOUND: &[u8] = include_bytes!("../sounds/fastinvader3.wav");
const INVADER_4_SOUND: &[u8] = include_bytes!("../sounds/fastinvader4.wav");
const INVADER_HIT_SOUND: &[u8] = include_bytes!("../sounds/ufo_lowpitch.wav");

pub struct Audio {
    _stream: OutputStream,
    sinks: Sinks,
    decoders: Decoders,
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

struct Decoders {
    ufo: Decoder<Cursor<&'static [u8]>>,
    shoot: Decoder<Cursor<&'static [u8]>>,
    player_death: Decoder<Cursor<&'static [u8]>>,
    invader_death: Decoder<Cursor<&'static [u8]>>,
    invader1: Decoder<Cursor<&'static [u8]>>,
    invader2: Decoder<Cursor<&'static [u8]>>,
    invader3: Decoder<Cursor<&'static [u8]>>,
    invader4: Decoder<Cursor<&'static [u8]>>,
    invader_hit: Decoder<Cursor<&'static [u8]>>,
}

impl Audio {
    pub fn try_new() -> Result<Self> {
        // TODO Sound constructor

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

        // Load audio files
        let decoders = Decoders {
            ufo: Decoder::new(Cursor::new(UFO_SOUND))?,
            shoot: Decoder::new(Cursor::new(SHOOT_SOUND))?,
            player_death: Decoder::new(Cursor::new(PLAYER_DEATH_SOUND))?,
            invader_death: Decoder::new(Cursor::new(INVADER_DEATH_SOUND))?,
            invader1: Decoder::new(Cursor::new(INVADER_1_SOUND))?,
            invader2: Decoder::new(Cursor::new(INVADER_2_SOUND))?,
            invader3: Decoder::new(Cursor::new(INVADER_3_SOUND))?,
            invader4: Decoder::new(Cursor::new(INVADER_4_SOUND))?,
            invader_hit: Decoder::new(Cursor::new(INVADER_HIT_SOUND))?,
        };


        
        todo!()
    }
}
