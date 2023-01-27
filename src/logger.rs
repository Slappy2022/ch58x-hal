use log::{max_level, Level, LevelFilter, Log, Metadata, Record};

pub fn init(level: LevelFilter) {
    if let Err(e) = log::set_logger(&LOGGER).map(|()| log::set_max_level(level)) {
        crate::println!("Error setting logger: {e}");
    }
}

static LOGGER: DefaultLogger = DefaultLogger;
struct DefaultLogger;
impl DefaultLogger {}
impl Log for DefaultLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= max_level()
    }

    fn log(&self, record: &Record) {
        let us = crate::now_us();
        let ms = us / 1_000;
        let secs = ms / 1_000;
        let mins = secs / 60;
        let hours = mins / 60;
        let days = hours / 24;

        let ms = ms % 1_000;
        let secs = secs % 60;
        let mins = mins % 60;
        let hours = hours % 24;

        let level = match record.level() {
            Level::Error => "E",
            Level::Warn => "W",
            Level::Info => "I",
            Level::Debug => "D",
            Level::Trace => "T",
        };
        crate::println!(
            "[{level} {days} {hours:0>2}:{mins:0>2}:{secs:0>2}.{ms:0>3}]\t {}",
            record.args()
        );
    }
    fn flush(&self) {}
}
