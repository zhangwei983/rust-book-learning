use chrono::Utc;
use slog::{self, info, Drain};
use std::{sync::Mutex, thread, time::Duration};

pub fn test() {
    let drain = Mutex::new(slog_json::Json::default(std::io::stdout())).fuse();
    let root_logger = slog::Logger::root(drain, slog::o!());

    info!(root_logger, "--- Start module: {}", module_path!());
    info!(root_logger, "Module started"; "started_at" => format!("{}", Utc::now()));
    thread::sleep(Duration::new(1, 0));
    info!(root_logger, "Module ended"; "ended_at" => format!("{}", Utc::now()));
    info!(root_logger, "--- End module: {}", module_path!());
}
