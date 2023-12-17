use crate::peripheral::Pp;
use std::error::Error;
use std::ffi::c_float;
use serialport::{available_ports, SerialPort, SerialPortType};
use crate::AirISP;
use rust_i18n::t;

pub struct Swd<'a> {
    air_isp: &'a AirISP::AirISP,

}

impl Swd<'_> {
    pub fn new(air_isp: &AirISP::AirISP) -> Swd {
        Swd {
            air_isp,
        }
    }
}

impl Pp for Swd<'_> {
    fn write_flash(&mut self, address: u32, data: &[u8], progress: AirISP::Progress) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn reset_bootloader(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn get_chip_id(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}