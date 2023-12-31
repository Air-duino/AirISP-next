use serde::Deserialize;
use std::error::Error;
include!(concat!(env!("OUT_DIR"), "/chips.rs"));

pub trait chip_info {
    fn get_chip_info(&self) -> Result<u32, Box<dyn Error>>;
}