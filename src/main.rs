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

fn set_language(air_isp: &AirISP::AirISP) {
    let language = if air_isp.get_language() != "auto" {
        air_isp.get_language().to_owned()
    } else {
        let language = whoami::lang().collect::<Vec<String>>();
        let language = language[0].as_str().to_owned();
        let i18n_list = rust_i18n::available_locales!();
        // 不支持的语言默认使用英语
        if !i18n_list.contains(&language.as_str()) {
            println!("{}","Language not supported");
            "en".to_owned()
        } else {
            language
        }
    };
    rust_i18n::set_locale(&language);
}

fn main() {

    let matches = AirISP::air_isp().get_matches();

    let air_isp = AirISP::AirISP::new(&matches);
    set_language(&air_isp);
    // 打印版本号
    println!("AirISP version: {}", env!("CARGO_PKG_VERSION").blue());
    
    if let Some((command, sub_m)) = matches.subcommand() {
        match command {
            "write_flash" => {
                let mut wf = write_flash::WriteFlash::new(&sub_m, air_isp);
                wf.run().unwrap();
            },
            "chip_id" => {
                let mut get = get::Get::new(&sub_m, air_isp);
                get.chip_id().unwrap();
            },
            _ => {
                println!("no subcommand");
            }
        }
    }
}
