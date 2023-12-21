use std::error::Error;
use std::io::Read;
use clap::{Arg, ArgMatches, ColorChoice, Command, value_parser};
use clap::builder::styling;
use rust_i18n::t;
use crate::{get, peripheral, write_flash};
use crate::peripheral::Pp;
use std::path::Path;
use std::string::String;

#[derive(PartialEq)]
pub enum Progress {
    None,
    Bar,
    Percent,
}

#[derive(Clone, Copy)]
pub enum Peripheral {
    GeneralUart,
    Swd,
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
        .value_parser(["Uart", "swd"])
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

#[derive(Clone)]
pub struct AirISP {
    port: String,
    baud: u32,
    chip: String,
    trace: bool,
    connect_attempts: u32,
    before: String,
    after: String,
    language: String,

    peripheral: Peripheral,
}

impl AirISP
{
    pub fn new(matches: &ArgMatches) -> AirISP
    {
        let pp = match matches.get_one::<String>("peripheral").unwrap().as_str() {
            "SWD" | "swd" => Peripheral::Swd,
            "GeneralUart" | "UART" | "uart" | _ => Peripheral::GeneralUart,
        };
        AirISP {
            port: matches.get_one::<String>("port").unwrap().to_string(),
            baud: *matches.get_one::<u32>("baud").unwrap(),
            trace: *matches.get_one::<bool>("trace").unwrap(),
            connect_attempts: *matches.get_one::<u32>("connect_attempts").unwrap(),
            before: matches.get_one::<String>("before").unwrap().to_string(),
            after: matches.get_one::<String>("after").unwrap().to_string(),
            language: matches.get_one::<String>("language").unwrap().to_string(),
            peripheral: pp,
            chip: matches.get_one::<String>("chip").unwrap().to_string(),
        }
    }

    pub fn get_peripheral(&self) -> Peripheral
    {
        self.peripheral
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

    pub fn read_file(&self, file_path: &str, address: &mut u32, bin: &mut Vec<u8>) -> Result<(), Box<dyn Error>>
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
        match suffix {
            "hex" => {
                let mut hex = String::new();
                file.read_to_string(&mut hex)?;
                self.hex_to_bin(hex, address, bin)?;
            }
            "bin" | _ => {
                file.read_to_end(bin)?;
            }
        }
        Ok(())
    }

    /// 如果是
    pub fn read_real_address(&mut self, address: &mut u32) -> Result<(), Box<dyn Error>>
    {

        Ok(())
    }


    /// 将 Intel Hex 格式的字符串转换为二进制数据
    ///
    /// # 参数
    /// * `hex` - Intel Hex 格式的字符串
    /// * `address` - 起始地址
    /// * `bin` - 要填充的二进制数据向量
    ///
    /// # 返回值
    /// 如果成功，返回 Ok(())；如果失败，返回错误信息
    pub fn hex_to_bin(&self, hex: String,address: &mut u32, bin: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        const FILL_BYTE: u8 = 0xFF;
        let mut start_address = String::from("-1");
        let mut current_address = 0;

        for line in hex.lines() {
            let hex_line = line.replace(":", "").replace(" ", "");
            let data_length = usize::from_str_radix(&hex_line[0..2], 16)?;
            let offset_address = usize::from_str_radix(&hex_line[2..6], 16)?;
            let record_type = usize::from_str_radix(&hex_line[6..8], 16)?;

            match record_type {
                0 => { // 数据记录
                    if start_address != "-1" {
                        if current_address < offset_address {
                            // 如果当前地址小于偏移地址，用 0xFF 填充间隙
                            bin.resize(offset_address, FILL_BYTE);
                        }
                        // 转换数据区的内容为二进制并存入 bin
                        for i in 0..data_length {
                            let data_byte = u8::from_str_radix(&hex_line[8 + i * 2..10 + i * 2], 16)?;
                            bin.push(data_byte);
                            current_address += 1;
                        }
                    }
                },
                1 => { // 文件结束记录
                    return Ok(());
                },
                4 => { // 扩展线性地址记录
                    // 获取高16位地址并更新起始地址
                    let high_address = &hex_line[8..12];
                    start_address = format!("0x{}0000", high_address);
                    *address = u32::from_str_radix(&start_address[2..], 16)?;
                    current_address = 0;
                },
                _ => { // 其他记录类型
                    // 忽略不处理
                }
            }
        }
        Ok(())
    }
}

// 实例化一个peripheral
#[macro_export]
macro_rules! instantiate_peripheral {
    ($air_isp:ident) => {
        match $air_isp.get_peripheral() {
            AirISP::Peripheral::Swd => {
                Box::new(peripheral::swd::Swd::new($air_isp)) as Box<dyn peripheral::Pp>
            },
            AirISP::Peripheral::GeneralUart | _ => {
                Box::new(peripheral::general_uart::GeneralUart::new($air_isp)) as Box<dyn peripheral::Pp>
            }
        }
    };
}
