use crate::peripheral::Pp;
use std::error::Error;
use crate::AirISP;
use rust_i18n::t;
use probe_rs::{Session, flashing, Permissions};
use probe_rs::flashing::DownloadOptions;
use colored::Colorize;

pub struct Swd<'a> {
    air_isp: &'a AirISP::AirISP,

}

impl Swd<'_> {
    pub fn new(air_isp: &AirISP::AirISP) -> Swd {
        Swd {
            air_isp,
        }
    }

    fn get_chip_session(&mut self) -> Result<Session, Box<dyn Error>> {
        let mut session;
        match self.air_isp.get_chip().as_str() {
            "air001" => {
                session = Session::auto_attach("Air001Dev", Permissions::default())?;
            }
            _ => {
                todo!()
            }
        }
        Ok(session)
    }
}

impl Pp for Swd<'_> {
    fn write_flash(&mut self, address: u32, data: &[u8], progress: AirISP::Progress) -> Result<(), Box<dyn Error>> {
        let mut session = self.get_chip_session()?;
        let mut loader = session.target().flash_loader();
        println!("{}",
                 format!("{}", t!("write_flash_file_help")).bright_blue()
        );
        let now_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        loader.add_data(address as u64, data)?;
        loader.commit(&mut session, DownloadOptions::default())?;
        println!("{}",
                 format!("{}",
                         t!("write_flash_success_help",
                       "time" => format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                           .unwrap()
                           .as_millis() - now_time),
                        "addr" => format!("{:#010x}", address as u32),
                        "size" => format!("{}", data.len())
                    )).bright_white()
        );
        Ok(())
    }
    fn reset_bootloader(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn get_chip_id(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn erase_all(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{}",
                 format!("{}", t!("erase_all_help")).bright_blue()
        );
        let now_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut session = self.get_chip_session()?;
        flashing::erase_all(&mut session, None)?;
        println!("{}",
                 format!("{}",
                         t!("erase_all_success_help",
                                   "time" => format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() - now_time)))
                     .green());
        Ok(())
    }
    fn reset_app(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{}",
                 format!("{}", t!("leaving_help")).white()
        );
        let mut session = self.get_chip_session()?;
        session.core(0)?.reset()?;
        Ok(())
    }
}