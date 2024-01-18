use std::{env, fs, path::Path};

use serde::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug)]
struct ChipFamily {
    family: String,
    chip: Vec<ChipInfo>,
}

#[derive(Debug)]
pub struct ChipInfo {
    pub name: String,
    pub debug_idcode_reg: u32,
    pub pid: u16,
    pub flash_size_reg: u32,
    pub flash_size: u32,
    pub ram_size: u32,
    pub uid_reg: u32,
}

fn parse_hex(s: &str) -> u32 {
    u32::from_str_radix(&s[2..], 16).unwrap()
}

fn hash_map_to_file(map: HashMap<String, ChipFamily>, dest_path: &PathBuf) {
    let struct_str = r#"use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct ChipInfo {
    pub name: &'static str,
    pub debug_idcode_reg: u32,
    pub pid: u16,
    pub flash_size_reg: u32,
    pub flash_size: u32,
    pub ram_size: u32,
    pub uid_reg: u32,
}

pub struct ChipFamily {
    family: &'static str,
    info: Vec<ChipInfo>,
}
    "#;

    // let mut chip = format!("pub CHIPS: [ChipFamily; {}] = [\n", map.len());
    let mut chip = String::new();
    chip.push_str("lazy_static! {\n");
    chip.push_str(format!("\tpub static ref CHIPS: [ChipFamily; {}] = [\n", map.len()).as_str());
    for (family, config) in map.iter() {
        chip.push_str(&format!(
            "\t\tChipFamily {{ \n",
        ));
        chip.push_str(&format!(
            "\t\t\tfamily: \"{}\", info: vec![\n",
            family
        ));
        for chip_info in config.chip.iter() {
            chip.push_str(&format!(
                "\t\t\t\tChipInfo {{ name: \"{}\", debug_idcode_reg: {:#X}, pid: {:#06X}, flash_size_reg: {:#X}, flash_size: {}, ram_size: {}, uid_reg: {:#X} }},\n",
                chip_info.name,
                chip_info.debug_idcode_reg,
                chip_info.pid,
                chip_info.flash_size_reg,
                chip_info.flash_size,
                chip_info.ram_size,
                chip_info.uid_reg,
            ));
        }
        chip.push_str("\t\t\t]\n \t\t},\n");
    }
    chip.push_str("\t];\n}\n");
    println!("{}" ,format!("{}\n{}", struct_str, chip));

    fs::write(dest_path, format!("{}\n{}", struct_str, chip)).unwrap();
}

fn creat_chip_info() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("chips.rs");
    let config_str =
        fs::read_to_string("./chip_info/config.toml").expect("Failed to read TOML file");
    let value = config_str.parse::<Value>().unwrap();
    let mut config_map = HashMap::new();
    // println!("{:#?}", value);
    for (family, chips) in value.as_table().unwrap().iter() {
        let mut chip_infos = Vec::new();

        for chip in chips.as_array().unwrap() {
            let chip_info = ChipInfo {
                name: chip.get("name").unwrap().as_str().unwrap().to_string(),
                debug_idcode_reg: chip.get("debug_idcode_reg").unwrap().as_integer().unwrap() as u32,
                pid: chip.get("pid").unwrap().as_integer().unwrap() as u16,
                flash_size_reg: chip.get("flash_size_reg").unwrap().as_integer().unwrap() as u32,
                flash_size: chip.get("flash_size").unwrap().as_integer().unwrap() as u32,
                ram_size: chip.get("ram_size").unwrap().as_integer().unwrap() as u32,
                uid_reg: chip.get("uid_reg").unwrap().as_integer().unwrap() as u32,
            };
            chip_infos.push(chip_info);
        }

        config_map.insert(family.to_string(), ChipFamily {
            family: family.to_string(),
            chip: chip_infos,
        });
    }
    hash_map_to_file(config_map, &dest_path);
}

fn main() {
    creat_chip_info();
    tauri_build::build()
}


