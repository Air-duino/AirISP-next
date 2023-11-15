use std::error::Error;
use clap::{Arg, ColorChoice, Command, value_parser};
use crate::language;

pub(crate) fn command(lang:&language::Strings) -> Command
{
    let erase = Arg::new("erase-all")
        .short('e')
        .long("erase-all")
        .help(lang.write_flash_erase_help.clone())
        .value_parser(value_parser!(bool))
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true");

    let no_progress = Arg::new("no-progress")
        .long("no-progress")
        .help(lang.no_progress_help.clone())
        .value_parser(value_parser!(bool))
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true");

    let address = Arg::new("address")
        .index(1)
        .required(true)
        .help(lang.write_flash_address_help.clone());

    let file_path = Arg::new("path")
        .index(2)
        .required(true)
        .help(lang.write_flash_file_path_help.clone());


    Command::new("write_flash")
        .about(lang.write_flash_help.clone())
        .color(ColorChoice::Auto)
        .arg(erase)
        .arg(no_progress)
        .arg(address)
        .arg(file_path)

}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>>
{

    Ok(())
}