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
}