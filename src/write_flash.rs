use std::error::Error;
use clap::{Arg, ColorChoice, Command, value_parser};
use clap::ArgMatches;
use crate::{AirISP, peripheral};
use rust_i18n::t;

pub fn command() -> Command
{
    let erase = Arg::new("erase-all")
        .short('e')
        .long("erase-all")
        .help(t!("write_flash_erase_help"))
        .value_parser(value_parser!(bool))
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true")
        .default_value("false");

    let no_progress = Arg::new("no-progress")
        .long("no-progress")
        .help(t!("no_progress_help"))
        .value_parser(value_parser!(bool))
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true")
        .default_value("false");

    let address = Arg::new("address")
        .id("address")
        .index(1)
        .required(true)
        .help(t!("write_flash_address_help"));

    let file_path = Arg::new("path")
        .id("path")
        .index(2)
        .required(true)
        .help(t!("write_flash_file_path_help"));


    Command::new("write_flash")
        .about(t!("write_flash_help"))
        .color(ColorChoice::Auto)
        .arg(erase)
        .arg(no_progress)
        .arg(address)
        .arg(file_path)

}

pub struct WriteFlash {
    address: u32,
    file_path: String,
    erase: bool,
    progress: AirISP::Progress,
    air_isp: AirISP::AirISP,
}

impl WriteFlash {
    pub fn new(matches: &ArgMatches, air_isp: AirISP::AirISP) -> WriteFlash
    {
        let address = matches.get_one::<String>("address").unwrap();
        let address = if address.starts_with("0x") || address.starts_with("0X") {
            u32::from_str_radix(&address[2..], 16).unwrap()
        } else {
            address.parse::<u32>().unwrap()
        };

        WriteFlash {
            address,
            file_path: matches.get_one::<String>("path").unwrap().to_string(),
            erase: *matches.get_one::<bool>("erase-all").unwrap(),
            progress: if *matches.get_one::<bool>("no-progress").unwrap() {
                AirISP::Progress::None
            } else {
                AirISP::Progress::Percent
            },

            air_isp,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>>
    {
        let air_isp = &self.air_isp;
        let mut binding = air_isp.get_peripheral_handle()?;
        let mut p = binding.get_pp();
        let mut bin = Vec::new();
        air_isp.read_file(&self.file_path,&mut self.address ,&mut bin)?;

        p.reset_bootloader()?;

        if self.erase {
            p.erase_all()?;
        }

        p.write_flash(self.address, &bin, AirISP::Progress::Percent)?;
        p.reset_app()?;
        Ok(())
    }
}
