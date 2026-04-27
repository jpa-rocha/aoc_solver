use crate::errors::{AppErrors, LoggerError};
use crate::options::get_options;

use spdlog::{prelude::*, sink::FileSink};

pub fn init_logs() -> Result<(), AppErrors> {
    let path = &get_options().log.path;

    let new_logger = match spdlog::default_logger().fork_with(|new| {
        let file_sink = FileSink::builder().path(path).build_arc()?;
        new.sinks_mut().push(file_sink);
        Ok(())
    }) {
        Ok(k) => k,
        Err(e) => return Err(LoggerError::CouldNotInitializeLogger(e).into()),
    };

    spdlog::set_default_logger(new_logger);

    spdlog::default_logger().set_level_filter(spdlog::LevelFilter::All);

    info!("from now on, logs will be written to both stdout/stderr and the file");
    Ok(())
}
