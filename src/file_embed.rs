use std::path::Path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./i18n/"]
struct Translations;

pub fn get_i18n() -> String {
    let language = whoami::lang().collect::<Vec<String>>();
    let path = format!("./i18n/{}.toml", language[0].as_str().to_owned());

    if Path::new(&path).exists() {
       return std::fs::read_to_string(path).unwrap();
    }
    let path = format!("{}.toml", language[0].as_str().to_owned());
    let t = Translations::get(&*path).unwrap();
    return String::from_utf8(t.data.to_vec()).unwrap();
}