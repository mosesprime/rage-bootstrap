use log::{Level, LevelFilter, Log, SetLoggerError};

static LOGGER: Logger = Logger;

struct Logger;

impl Log for Logger {
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => println!("[\x1b[0;31mERROR\x1b[0m] {}", record.args()),
                Level::Warn => println!("[\x1b[0;33mWARN\x1b[0m]  {}", record.args()),
                Level::Info => println!("[\x1b[0;32mINFO\x1b[0m]  {}", record.args()),
                Level::Debug => println!("[\x1b[0;34mDEBUG\x1b[0m] {}", record.args()),
                Level::Trace =>  println!("[TRACE] {}", record.args()),
            }
        } 
    }

    fn flush(&self) {}

    fn enabled(&self, metadata: &log::Metadata) -> bool {
        #[cfg(debug_assertions)]
        return metadata.level() <= Level::Debug;
        #[cfg(not(debug_assertions))]
        return metadata.level() <= Level::Info;
    }
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    #[cfg(debug_assertions)]
    log::set_max_level(LevelFilter::Debug);
    #[cfg(not(debug_assertions))]
    log::set_max_level(LevelFilter::Info);
    log::set_logger(&LOGGER)
}
