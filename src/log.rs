use colored::{Color, Colorize};
use lazy_static::lazy_static;

pub enum Level {
    Trace,
    Info,
    Warn,
    Error,
}

pub struct Log  {
    level: Level,
}

impl Log {
    pub fn new(level: Level) -> Log {
        Log {
            level,
        }
    }

    pub fn info(&self, msg: &str, color: Color) {
        match self.level {
            Level::Info | Level::Trace => {
                println!("{}", msg.color(color));
            },
            _ => {},
        }
    }

    pub fn info_no_color(&self, msg: &str) {
        self.info(msg, Color::White);
    }

    pub fn error(&self, msg: &str) {
        match self.level {
            _ => {
                let level = "ERROR: ".color(Color::Red);
                let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().color(Color::White);
                eprintln!("{} [{}] {}",
                         level,
                         time,
                         msg
                );
            },
        }
    }

    pub fn warn(&self, msg: &str) {
        match self.level {
            Level::Warn | Level::Info | Level::Trace => {
                let level = "Warn: ".color(Color::Yellow);
                let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().color(Color::White);
                println!("{} [{}] {}",
                         level,
                         time,
                         msg
                );
            },
            _ => {},
        }
    }

    pub fn trace(&self, msg: &str) {
        match self.level {
            Level::Trace => {
                let level = "Trace: ".color(Color::Cyan);
                let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().color(Color::White);
                println!("{} [{}] {}",
                         level,
                         time,
                         msg
                );
            },
            _ => {},
        }
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }
}

lazy_static! {
    pub static ref LOG: Log = Log {
        level: Level::Info,
    };
}