use std::error::Error;
use clap::{ColorChoice, Command};
use clap::ArgMatches;
use crate::{AirISP, peripheral};
use rust_i18n::t;

pub fn chip_id_command() -> Command {
    Command::new("chip_id")
        .about(t!("chip_id_help"))
        .color(ColorChoice::Auto)
}

pub struct Get {
    air_isp: AirISP::AirISP,
}

impl Get {
    pub fn new(_: &ArgMatches, air_isp: AirISP::AirISP) -> Get {
        Get {
            air_isp: air_isp,
        }
    }

    pub fn chip_id(&mut self) -> Result<(), Box<dyn Error>> {
        let air_isp = &self.air_isp;
        let mut binding = air_isp.get_peripheral_handle()?;
        let mut peripheral = binding.get_pp();
        peripheral.get_chip_id()?;
        Ok(())
    }
}
