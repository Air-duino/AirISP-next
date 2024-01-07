#![allow(non_snake_case)]
use std::error::Error;
use std::io::Read;
use clap::{Arg, ArgMatches, ColorChoice, Command, value_parser};
use clap::builder::styling;
use rust_i18n::t;
use crate::{get, hex_to_bin, write_flash};
use std::path::Path;
use std::string::String;
use crate::peripheral;

#[derive(PartialEq)]
pub enum Progress {
    None,
    Bar,
    Percent,
}

pub fn air_isp() -> Command
{
    let port = Arg::new("port")
        .global(true)
        .short('p')
        .long("port")
        .help(t!("port_help"))
        .default_value("auto")
        .required(false);

    let chip = Arg::new("chip")
        .global(true)
        .short('c')
        .long("chip")
        .help(t!("chip_help"))
        .value_parser(["auto", "air001", "air32"])
        .default_value("auto");

    let baud = Arg::new("baud")
        .global(true)
        .short('b')
        .long("baud")
        .help(t!("baud_help"))
        .value_parser(value_parser! { u32 })
        .default_value("115200");

    let trace = Arg::new("trace")
        .global(true)
        .short('t')
        .long("trace")
        .help(t!("trace_help"))
        .value_parser(value_parser!(bool))
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true")
        .default_value("false");

    let connect_attempts = Arg::new("connect_attempts")
        .global(true)
        .long("connect-attempts")
        .help(t!("connect_attempts_help"))
        .value_parser(value_parser! { u32 })
        .default_value("10");

    let before = Arg::new("before")
        .global(true)
        .long("before")
        .help(t!("before_help"))
        .value_parser(["direct_connect", "default_reset"])
        .default_value("default_reset");

    let after = Arg::new("after")
        .global(true)
        .long("after")
        .help(t!("after_help"))
        .value_parser(["hard_reset"])
        .default_value("hard_reset");

    let peripheral = Arg::new("peripheral")
        .global(true)
        .long("peripheral")
        .help(t!("peripheral_help"))
        .default_value("Uart");

    let language = Arg::new("language")
        .global(true)
        .long("language")
        .help(t!("language_help"))
        .value_parser(["auto", "en", "zh-CN", "ja"])
        .default_value("auto");

    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());

    Command::new("AirISP")
        .about(t!("root_help"))
        .color(ColorChoice::Auto)
        .styles(styles)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(port)
        .arg(chip)
        .arg(baud)
        .arg(trace)
        .arg(connect_attempts)
        .arg(before)
        .arg(after)
        .arg(peripheral)
        .arg(language)
        .subcommand(write_flash::command())
        .subcommand(get::chip_id_command())
}

pub struct AirISP {
    port: String,
    baud: u32,
    chip: String,
    trace: bool,
    connect_attempts: u32,
    before: String,
    after: String,
    language: String,
    peripheral: String,
}

impl AirISP
{
    pub fn new(matches: &ArgMatches) -> AirISP
    {
        AirISP {
            port: matches.get_one::<String>("port").unwrap().to_string(),
            baud: *matches.get_one::<u32>("baud").unwrap(),
            trace: *matches.get_one::<bool>("trace").unwrap(),
            connect_attempts: *matches.get_one::<u32>("connect_attempts").unwrap(),
            before: matches.get_one::<String>("before").unwrap().to_string(),
            after: matches.get_one::<String>("after").unwrap().to_string(),
            language: matches.get_one::<String>("language").unwrap().to_string(),
            chip: matches.get_one::<String>("chip").unwrap().to_string(),
            peripheral: matches.get_one::<String>("peripheral").unwrap().to_string(),
        }
    }

    pub fn get_port(&self) -> String
    {
        self.port.clone()
    }
    pub fn get_baud(&self) -> u32
    {
        self.baud
    }

    pub fn get_trace(&self) -> bool
    {
        self.trace
    }

    pub fn get_connect_attempts(&self) -> u32
    {
        self.connect_attempts
    }

    pub fn get_before(&self) -> String
    {
        self.before.clone()
    }

    pub fn get_after(&self) -> String
    {
        self.after.clone()
    }

    pub fn get_language(&self) -> String
    {
        self.language.clone()
    }

    pub fn get_chip(&self) -> String
    {
        self.chip.clone()
    }

    pub fn get_peripheral(&self) -> String
    {
        self.peripheral.clone()
    }

    pub fn get_peripheral_handle(& self) -> Result<peripheral::Peripheral, Box<dyn Error>>
    {
        // 全部转换为小写
        let peripheral = self.get_peripheral().to_lowercase();
        match peripheral.as_str() {
            "swd" => {
                let p = peripheral::Peripheral::Swd(peripheral::swd::Swd::new(&self));
                Ok(p)
            },
            "uart" => {
                let p = peripheral::Peripheral::GeneralUart(peripheral::general_uart::GeneralUart::new(&self));
                Ok(p)
            },
            _ => {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "not support peripheral")))
            }
        }
    }

    pub fn read_file(&self, file_path: &str) -> Result<Vec<hex_to_bin::Bin>, Box<dyn Error>>
    {
        // 判断文件后缀是.hex还是.bin
        let mut file = std::fs::File::open(file_path)?;
        // 如果文件为空，直接返回
        if file.metadata().unwrap().len() == 0 {
           return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "file is empty")));
        }
        // 读取后缀名
        let path = Path::new(file_path);
        let suffix = path.extension().unwrap().to_str().unwrap_or("");
        let vec_bin = match suffix {
            "hex" => {
                let mut hex = String::new();
                file.read_to_string(&mut hex)?;
                hex_to_bin::hex_to_bin(&hex)?
            }
            "bin" | _ => {
                let mut bin = Vec::new();
                file.read_to_end(&mut bin)?;
                vec![hex_to_bin::Bin {
                    address: 0xFFFF_FFFF,
                    data: bin,
                }]
            }
        };
        Ok(vec_bin)
    }
}
