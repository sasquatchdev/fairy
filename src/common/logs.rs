use crate::*;

/// function that initializes the `env_logger`
/// and `log` crates to enable logging in this
/// application.
/// 
/// make sure to call this function near the beginning
/// of the lifecycle of your application, as any logs
/// that are generated before this function is called 
/// will effectively be lost.

pub fn init() -> ResultVoid {
    // initialize the env_logger
    let result = env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(true)
        .try_init();

    if let Err(e) = result {
        eprintln!("failed to initialize logs. {}", e);
        return Err(e.into());
    }

    // log feedback
    log::info!(
    "logging initialized.");
    Ok(())
}

/// function that returns the current time,
/// formatted as a nice, uniform and readable
/// string.

pub fn time() -> String {
    let now = chrono::Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    formatted_time
}
