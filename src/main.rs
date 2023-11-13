mod language;
mod file_embed;

use clap::error::Error;
use clap::{Arg, ArgAction, ArgMatches, Args, ColorChoice, Command, command, FromArgMatches, Parser, value_parser};
use clap::builder::{styling, ValueParser};
use toml::Value;

fn write_flash(language::Strings)
{

}

fn AirISP(lang:language::Strings) -> Command
{
    let port = Arg::new("port")
        .global(true)
        .short('p')
        .long("port")
        .help(lang.port_help)
        .required(false);

    let chip = Arg::new("chip")
        .global(true)
        .short('c')
        .long("chip")
        .help(lang.chip_help)
        .required(true)
        .value_parser(["auto", "air001","air32"])
        .default_value("auto");

    let baud = Arg::new("baud")
        .global(true)
        .short('b')
        .long("baud")
        .help(lang.baud_help)
        .required(true)
        .value_parser(value_parser!{ u32 })
        .default_value("115200");

    let trace = Arg::new("trace")
        .global(true)
        .short('t')
        .long("trace")
        .help(lang.trace_help)
        .required(false)
        .value_parser(value_parser!{ bool })
        .default_value("false");

    let connect_attempts = Arg::new("connect_attempts")
        .global(true)
        .long("connect-attempts")
        .help(lang.connect_attempts_help)
        .required(false)
        .value_parser(value_parser!{ u32 })
        .default_value("10");

    let before = Arg::new("before")
        .global(true)
        .long("before")
        .help(lang.before_help)
        .required(true)
        .value_parser(["direct_connect", "default_reset"])
        .default_value("default_reset");

    let after = Arg::new("after")
        .global(true)
        .long("after")
        .help(lang.after_help)
        .required(true)
        .value_parser(["hard_reset"])
        .default_value("hard_reset");

    let peripheral = Arg::new("peripheral")
        .global(true)
        .long("peripheral")
        .help(lang.peripheral_help)
        .required(true)
        .value_parser(["uart", "swd"])
        .default_value("uart");

    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());

    Command::new("AirISP")
        .about(lang.root_help)
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

}


fn main() {

    let my_string : language::Strings = toml::from_str(&*file_embed::get_i18n()).unwrap();

    let matches = AirISP(my_string).get_matches();
}