use chrono::Utc;
use slog::{self, info, Drain};
use std::{fs::OpenOptions, thread, time::Duration};

pub fn test() {
    let log_path = "log_file.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let root_logger = slog::Logger::root(drain, slog::o!());

    info!(root_logger, "--- Start module: {}", module_path!());
    info!(root_logger, "Module started"; "started_at" => format!("{}", Utc::now()));
    thread::sleep(Duration::new(1, 0));
    info!(root_logger, "Module ended"; "ended_at" => format!("{}", Utc::now()));
    info!(root_logger, "--- End module: {}", module_path!());
}
