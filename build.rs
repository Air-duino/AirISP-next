use std::{env, fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    chip: Vec<ChipInfo>,
}

#[derive(Deserialize)]
pub struct ChipInfo {
    pub name: String,
    pub debug_idcode_reg: u32,
    pub pid: u16,
    pub flash_size_reg: u32,
    pub flash_size: u32,
    pub ram_size: u32,
    pub uid_reg: u32,
}

fn creat_chip_info() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("chips.rs");
    let config_str =
        fs::read_to_string("./chip_info/config.toml").expect("Failed to read TOML file");

    let config: Config = toml::from_str(&config_str).expect("Failed to parse TOML file");

    let chips_str = config
        .chip
        .iter()
        .map(|chip| {
            let Series =
            format!(
                "
    ChipInfo {{ 
        name: \"{}\", 
        debug_idcode_reg: {:#X}, 
        pid: {:#X}, 
        flash_size_reg: {:#X}, 
        uid_reg: {:#X}, 
    }}",
                chip.name,
                chip.debug_idcode_reg,
                chip.pid,
                chip.flash_size_reg,
                chip.uid_reg
            );
            Series
        })
        .collect::<Vec<_>>()
        .join(", ");
    
    let chip_struct = "#[derive(Deserialize)]
pub struct ChipInfo {
    pub name: &'static str,
    pub debug_idcode_reg: u32,
    pub pid: u16,
    pub flash_size_reg: u32,
    pub uid_reg: u32,
}
";
    fs::write(
        &dest_path,
        format!(
            "{}
pub const CHIPS: [ChipInfo; {}] = [{}];",
            chip_struct,
            config.chip.len(),
            chips_str
        ),
    )
    .expect("Failed to write chips file");
}

fn main() {
    creat_chip_info();
}
