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
            air_isp: air_isp.clone(),
        }
    }

    pub fn chip_id(&mut self) -> Result<(), Box<dyn Error>> {
        let air_isp = &self.air_isp;
        use crate::instantiate_peripheral;
        let mut p  = instantiate_peripheral!(air_isp);
        p.get_chip_id()?;
        Ok(())
    }
}
