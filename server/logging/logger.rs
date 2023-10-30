use crate::logging::Color;

use chrono::prelude::Local;

pub enum Level {
    DEBUG,
    SUCCESS,
    INFO,
    WARNING,
    ERROR,
}

impl Level {
    pub fn get_color(&self) -> Color {
        match *self {
            Level::DEBUG => Color::CYAN,
            Level::SUCCESS => Color::GREEN,
            Level::INFO => Color::WHITE,
            Level::WARNING => Color::YELLOW,
            Level::ERROR => Color::RED,
        }
    }

    pub fn get_prio(&self) -> i32 {
        match *self {
            Level::DEBUG => -2,
            Level::SUCCESS => -1,
            Level::INFO => 0,
            Level::WARNING => 1,
            Level::ERROR => 2,
        }
    }

    pub fn get_name(&self) -> &str {
        match *self {
            Level::DEBUG => "DEBUG",
            Level::SUCCESS => "SUCCESS",
            Level::INFO => "INFO",
            Level::WARNING => "WARNING",
            Level::ERROR => "ERROR",
        }
    }
}

static mut MINIMUM_LEVEL: Level = Level::INFO;

pub fn set_log_level(min_level: Level) {
    unsafe {
        MINIMUM_LEVEL = min_level;
    }
}

pub fn log<StringLike: AsRef<str>>(level: Level, content: StringLike) {
    println!("{}", log_str(level, content));
}

pub fn log_str<StringLike: AsRef<str>>(level: Level, content: StringLike) -> String {
    if level.get_prio() < unsafe { MINIMUM_LEVEL.get_prio() } {
        return String::new();
    }
    let time = Local::now();
    let result = format!(
        "{}[{} {}] {}{}",
        level.get_color().to_string(),
        time.format("%H:%M:%S").to_string(),
        level.get_name(),
        content.as_ref(),
        Color::RESET.to_string()
    );
    result
}

pub fn debug<StringLike: AsRef<str>>(content: StringLike) {
    log(Level::DEBUG, content.as_ref());
}

pub fn success<StringLike: AsRef<str>>(content: StringLike) {
    log(Level::SUCCESS, content.as_ref());
}

pub fn info<StringLike: AsRef<str>>(content: StringLike) {
    log(Level::INFO, content.as_ref());
}

pub fn warning<StringLike: AsRef<str>>(content: StringLike) {
    log(Level::WARNING, content.as_ref());
}
pub fn error<StringLike: AsRef<str>>(content: StringLike) {
    log(Level::ERROR, content.as_ref());
}

pub fn debug_str<StringLike: AsRef<str>>(content: StringLike) -> String {
    log_str(Level::DEBUG, content.as_ref())
}

pub fn success_str<StringLike: AsRef<str>>(content: StringLike) -> String {
    log_str(Level::SUCCESS, content.as_ref())
}

pub fn info_str<StringLike: AsRef<str>>(content: StringLike) -> String {
    log_str(Level::INFO, content.as_ref())
}

pub fn warning_str<StringLike: AsRef<str>>(content: StringLike) -> String {
    log_str(Level::WARNING, content.as_ref())
}
pub fn error_str<StringLike: AsRef<str>>(content: StringLike) -> String {
    log_str(Level::ERROR, content.as_ref())
}

pub fn debug_panic<StringLike: AsRef<str>>(content: StringLike) {
    panic!("{}", log_str(Level::DEBUG, content.as_ref()))
}

pub fn success_panic<StringLike: AsRef<str>>(content: StringLike) {
    panic!("{}", log_str(Level::SUCCESS, content.as_ref()))
}

pub fn info_panic<StringLike: AsRef<str>>(content: StringLike) {
    panic!("{}", log_str(Level::INFO, content.as_ref()))
}

pub fn warning_panic<StringLike: AsRef<str>>(content: StringLike) {
    panic!("{}", log_str(Level::WARNING, content.as_ref()))
}
pub fn error_panic<StringLike: AsRef<str>>(content: StringLike) {
    panic!("{}", log_str(Level::ERROR, content.as_ref()))
}
