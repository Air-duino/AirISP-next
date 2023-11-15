use serde_derive::Deserialize;
// 国际化字符串
#[derive(Deserialize)]
pub struct Strings {
    pub root_help: String,
    pub port_help: String,
    pub chip_help: String,
    pub baud_help: String,
    pub trace_help: String,
    pub connect_attempts_help: String,
    pub before_help: String,
    pub after_help: String,
    pub peripheral_help: String,
    pub write_flash_help: String,
    pub write_flash_erase_help: String,
    pub no_progress_help: String,
    pub write_flash_address_help: String,
    pub write_flash_file_path_help: String,
}