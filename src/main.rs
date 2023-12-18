mod write_flash;
mod peripheral;
mod AirISP;
mod get;

use rust_i18n::t;
use colored::*;

rust_i18n::i18n!("i18n");

use clap::error::Error;
use clap::{Arg, ArgAction, ArgMatches, Args, ColorChoice, Command, command, FromArgMatches, Parser, value_parser};
use clap::builder::{styling, ValueParser};

fn get_system_language() -> String {
    let language = whoami::lang().collect::<Vec<String>>();
    let language = language[0].as_str().to_owned();
    language
}

fn default_language()  {
    let language = get_system_language();
    let i18n_list = rust_i18n::available_locales!();
    if !i18n_list.contains(&language.as_str()) {
        rust_i18n::set_locale("en");
    }
    else {
        rust_i18n::set_locale(&language);
    }
}

fn set_language(air_isp: &AirISP::AirISP) {
    let language = get_system_language();
    let i18n_list = rust_i18n::available_locales!();

    if air_isp.get_language() != "auto" {
        rust_i18n::set_locale(&air_isp.get_language());
        return;
    }
    // 不支持的语言默认使用英语
    if !i18n_list.contains(&language.as_str()) {
        rust_i18n::set_locale("en");
    }
    else {
        rust_i18n::set_locale(&language);
    }
}

fn main() {
    default_language();
    let matches = AirISP::air_isp().get_matches();

    let air_isp = AirISP::AirISP::new(&matches);
    set_language(&air_isp);
    // 打印版本号
    println!("{}", format!("AirISP version: {}", env!("CARGO_PKG_VERSION")).blue());
    
    match matches.subcommand() {
        Some(("write_flash", sub_m)) => {
            let mut wf = write_flash::WriteFlash::new(&sub_m, air_isp);
            wf.run().unwrap();
        },
        Some(("chip_id", sub_m)) => {
            let mut get = get::Get::new(&sub_m, air_isp);
            get.chip_id().unwrap();

        },
        _ => {
            println!("no subcommand");
        }
    }
}