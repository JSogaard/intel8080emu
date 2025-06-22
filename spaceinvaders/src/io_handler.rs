use crate::{
    audio::Audio, errors::{Error, Result}, shift_register::ShiftRegister
};
use intel8080_core::port::Port;
use sdl2::keyboard::Keycode;

pub struct IoHandler {
    buttons: Buttons,
    audio: Audio,
    shift_regster: ShiftRegister,
    dip_shipnum: u8,
    dip_extraship_point: bool,
}

#[derive(Clone, Debug, Default)]
struct Buttons {
    p1_start: bool,
    p1_left: bool,
    p1_right: bool,
    p1_shoot: bool,

    p2_start: bool,
    p2_left: bool,
    p2_right: bool,
    p2_shoot: bool,
}

impl IoHandler {
    pub fn try_new(dip_settings: (u8, bool)) -> Result<Self> {
        if dip_settings.0 > 3 {
            return Err(Error::InvalidDipInput);
        }

        Ok(Self {
            buttons: Buttons::default(),
            audio: Audio::try_new()?,
            shift_regster: ShiftRegister::default(),
            dip_shipnum: dip_settings.0,
            dip_extraship_point: dip_settings.1,
        })
    }

    pub fn set_key(&mut self, key: Keycode, value: bool) {
        match key {
            Keycode::Num1 => self.buttons.p1_start = value,
            Keycode::A => self.buttons.p1_left = value,
            Keycode::D => self.buttons.p1_right = value,
            Keycode::Space => self.buttons.p1_shoot = value,

            Keycode::Num2 => self.buttons.p2_start = value,
            Keycode::J => self.buttons.p2_left = value,
            Keycode::L => self.buttons.p2_right = value,
            Keycode::RShift => self.buttons.p2_shoot = value,

            _ => {}
        }
    }
}

impl Port for IoHandler {
    fn read_in(&self, port_num: u8) -> u8 {
        match port_num {
            1 => {
                // Insert bits from state
                0b00001001
                    | ((self.buttons.p2_start as u8) << 1)
                    | ((self.buttons.p1_start as u8) << 2)
                    | ((self.buttons.p1_shoot as u8) << 4)
                    | ((self.buttons.p1_left as u8) << 5)
                    | ((self.buttons.p1_right as u8) << 6)
            }

            2 => {
                // Insert bits from state
                0b10000000
                    | ((self.dip_shipnum & 0b10) >> 1)
                    | ((self.dip_shipnum & 0b01) << 1)
                    | ((self.dip_extraship_point as u8) << 3)
                    | ((self.buttons.p2_shoot as u8) << 4)
                    | ((self.buttons.p2_left as u8) << 5)
                    | ((self.buttons.p2_right as u8) << 6)
            }

            3 => self.shift_regster.read(),

            _ => panic!("Invalid port number was read IN: {port_num}"),
        }
    }

    fn write_out(&mut self, port_num: u8, value: u8) {
        match port_num {
            2 => self.shift_regster.set_offset(value),
            3 => self.audio.play_port3(value).expect("An error occured while playing game audio"),
            4 => self.shift_regster.insert(value),
            5 => self.audio.play_port5(value).expect("An error occured while playing game audio"),
            _ => panic!("Invalid port number was written OUT: {port_num}")
        }
    }
}
