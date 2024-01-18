#![allow(non_snake_case)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod write_flash;
mod peripheral;
mod AirISP;
mod get;
mod hex_to_bin;
mod log;

use colored::*;

rust_i18n::i18n!("i18n");

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn default_language() {
    let language = whoami::lang().collect::<Vec<String>>();
    let language = language[0].as_str().to_owned();
    let i18n_list = rust_i18n::available_locales!();
    // 不支持的语言默认使用英语
    if !i18n_list.contains(&language.as_str()) {
        println!("{}","Language not supported");
        rust_i18n::set_locale("en");
    } else {
        rust_i18n::set_locale(&language);
    }
}

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

fn cli() {
    default_language();
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        #[cfg(windows)]
        {
            use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
            // we ignore the result here because
            // if the app started from a command line, like cmd or powershell,
            // it will attach sucessfully which is what we want
            // but if we were started from something like explorer,
            // it will fail to attach console which is also what we want.
            let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
        }
        cli();
        return;
    }
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
