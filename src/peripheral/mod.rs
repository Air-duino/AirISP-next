pub mod general_uart;
pub mod swd;
use std::error::Error;
use serialport::{available_ports, SerialPort, SerialPortType};
use crate::AirISP;

pub trait Pp {
    fn write_flash(&mut self ,address: u32, data: &[u8], progress:AirISP::Progress) -> Result<(), Box<dyn Error>>;

    /// 重启到bootloader
    fn reset_bootloader(&mut self) -> Result<(), Box<dyn Error>>;

    /// 重启到APP
    fn reset_app(&mut self) -> Result<(), Box<dyn Error>>;

    /// 获取芯片ID
    fn get_chip_id(&mut self) -> Result<(), Box<dyn Error>>;

    /// 擦除全片
    fn erase_all(&mut self) -> Result<(), Box<dyn Error>>;
}

