use log::{Log, LogRecord, LogLevel, LogMetadata, SetLoggerError};

pub fn setup(level: LogLevel) -> Result<(), SetLoggerError>{
	use log;
	log::set_logger(|filter|{
		filter.set(level.to_log_level_filter());
		Box::new(Logger(level))
	})
}

pub fn setup_default() -> Result<(), SetLoggerError>{
	setup(LogLevel::Debug)
}

pub struct Logger(LogLevel);
impl Log for Logger{
	fn enabled(&self, metadata: &LogMetadata) -> bool{
		metadata.level() <= self.0
	}
	fn log(&self, record: &LogRecord){
		if self.enabled(record.metadata()){
			let location = record.location();
			println!("[{:?}][{}::{}:{}] {}", record.level(), location.module_path(), location.file(), location.line(), record.args());
		}
	}
}
