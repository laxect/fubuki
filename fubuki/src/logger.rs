use gloo_console as console;
use log::{Level, Log, Metadata, Record, SetLoggerError};

pub struct Config {
    pub level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Config { level: Level::Warn }
    }
}

impl Config {
    #[cfg(debug_assertions)]
    fn debug_profile() -> Self {
        Config { level: Level::Debug }
    }
}

static LOGGER: WebLogger = WebLogger;

struct WebLogger;

impl Log for WebLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // TODO Check the args of a location
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if self.enabled(metadata) {
            let msg = format!("{}:{} -- {}", record.level(), record.target(), record.args());
            match metadata.level() {
                Level::Trace => console::trace!(&msg),
                Level::Debug => console::debug!(&msg),
                Level::Info => console::info!(&msg),
                Level::Warn => console::warn!(&msg),
                Level::Error => console::error!(&msg),
            }
        }
    }

    fn flush(&self) {}
}

fn try_init(config: Config) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    let level = config.level;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

#[cfg(not(debug_assertions))]
pub fn init() {
    try_init(Config::default()).expect("web_logger::init should not be called after logger initialized");
}

#[cfg(debug_assertions)]
pub fn init() {
    try_init(Config::debug_profile()).expect("web_logger::init should not be called after logger initialized");
}
