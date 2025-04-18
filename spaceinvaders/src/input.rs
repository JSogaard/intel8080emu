use intel8080_core::port::Port;

pub struct Input {

}

impl Input {
    pub fn new() -> Self {
        todo!()
    }
}

impl Port for Input {
    fn read_in(&self, port_num: u8) -> u8 {
        todo!()
    }

    fn write_out(&mut self, port_num: u8, value: u8) {
        todo!()
    }
}