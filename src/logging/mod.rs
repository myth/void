use ansi_term::Color::{Red, Yellow};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

pub struct TinyLogger;

static LOGGER: TinyLogger = TinyLogger;

impl log::Log for TinyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{}", record.args());

            match record.level() {
                Level::Error => eprintln!("{} {}", Red.paint("ERROR:"), Red.paint(message)),
                Level::Warn => println!("{} {}", Yellow.paint("WARNING:"), Yellow.paint(message)),
                _ => println!("{}", record.args()),
            }
        }
    }

    fn flush(&self) {}
}

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}
