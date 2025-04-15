pub trait Port {
    fn read_in(&self, port_num: u8) -> u8;
    fn write_out(&mut self, port_num: u8, value: u8);
}