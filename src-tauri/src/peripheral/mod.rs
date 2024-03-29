pub mod general_uart;
pub mod swd;
use std::error::Error;
use crate::{AirISP, peripheral};

include!(concat!(env!("OUT_DIR"), "/chips.rs"));

pub trait chip_info {
    fn get_chip_info(&mut self) -> Result<&ChipInfo, Box<dyn Error>>;
    fn get_chip_pid(&mut self) -> Result<u32, Box<dyn Error>>;
}

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

pub enum Peripheral<'a> {
    Swd(peripheral::swd::Swd<'a>),
    GeneralUart(peripheral::general_uart::GeneralUart<'a>),
}

impl<'a> Peripheral<'a> {
    pub fn get_pp(&mut self) -> &mut dyn Pp {
        match self {
            Peripheral::Swd(pp) => pp,
            Peripheral::GeneralUart(pp) => pp,
        }
    }
}