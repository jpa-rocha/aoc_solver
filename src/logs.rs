use crate::errors::{AppErrors, LoggerError};
use crate::options::get_options;

use spdlog::sink::FileSink;

pub fn init_logs() -> Result<(), AppErrors> {
    match get_options().logs.to_file {
        true => {
            let path = &get_options().logs.path.join("aoc_solver.log");
            let new_logger = match spdlog::default_logger().fork_with(|new| {
                let file_sink = FileSink::builder().path(path).build_arc()?;
                new.sinks_mut().push(file_sink);
                Ok(())
            }) {
                Ok(k) => k,
                Err(e) => return Err(LoggerError::CouldNotInitializeLogger(e).into()),
            };

            spdlog::set_default_logger(new_logger);
        }
        false => {}
    }

    let log_level = match get_options().logs.level.to_lowercase().as_str() {
        "trace" => spdlog::Level::Trace,
        "debug" => spdlog::Level::Debug,
        "info" => spdlog::Level::Info,
        "warn" => spdlog::Level::Warn,
        "error" => spdlog::Level::Error,
        "critical" => spdlog::Level::Critical,
        _ => return Err(LoggerError::UnsupportedLoggerLevel.into()),
    };

    spdlog::default_logger().set_level_filter(spdlog::LevelFilter::MoreSevereEqual(log_level));

    Ok(())
}
