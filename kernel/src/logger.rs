use core::cmp::PartialEq;
use crate::serial;
use crate::serial::Serial;

pub struct Logger {
    serial: Serial,
}

#[derive(Clone, Copy)]
pub enum LogLevel{
    Error,
    Warn,
    Info,
}

impl LogLevel {
    pub (crate) fn new(level : u8) -> LogLevel{
        if level == 1{
            return LogLevel::Error
        }
        else if level == 2{
            return LogLevel::Warn
        }
        else if level == 3{
            return LogLevel::Info
        }
        panic!("Unknown log level {}", level);
    }
}

impl PartialEq for LogLevel {

    fn eq(&self, other: &Self) -> bool {
        if let (LogLevel::Warn, LogLevel::Warn) = (self, other) {
            return true;
        }
        else if let (LogLevel::Info, LogLevel::Info) = (self, other) {
            return true;
        }
        else if let (LogLevel::Error, LogLevel::Error) = (self, other) {
            return true;
        }
        return false;
    }
}

impl Logger {
    pub(crate) fn new(serial: Serial) -> Logger {
        Logger { serial }
    }
    pub(crate) fn boot_log(mut self) {
        Serial::write_message(&mut self.serial, "[INFO] Kernel booted\n\
        [INFO] serial initialised \n\
        [INFO] framebuffer initialised\n");
    }
    pub(crate) fn log(&mut self, message: &str, level: LogLevel) {
        if (level == LogLevel::Info) {
            Serial::write_message(&mut self.serial, "\x1b[32m");
            Serial::write_message(&mut self.serial, "[INFO]");
            Serial::write_message(&mut self.serial, "\x1b[0m");
        } else if (level == LogLevel::Error) {
            Serial::write_message(&mut self.serial, "\x1b[31m");
            Serial::write_message(&mut self.serial, "[ERROR]");
            Serial::write_message(&mut self.serial, "\x1b[0m");
        } else if (level == LogLevel::Warn) {
            Serial::write_message(&mut self.serial, "\x1b[33m");
            Serial::write_message(&mut self.serial, "[WARNING]");
            Serial::write_message(&mut self.serial, "\x1b[0m");
        }
        Serial::write_message(&mut self.serial, message);
        Serial::write_message(&mut self.serial, "\n");
    }
}


