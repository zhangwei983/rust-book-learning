use chrono::Utc;
use slog::{self, info, Drain};
use std::fs::OpenOptions;

pub fn test() {
    // Log to terminal.
    let decorator = slog_term::TermDecorator::new().build();
    let drain1 = slog_term::FullFormat::new(decorator).build().fuse();
    let drain1 = slog_async::Async::new(drain1).build().fuse();

    let log_path = "log_file_from_multiple_drains.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    // Log to file.
    let decorator = slog_term::PlainDecorator::new(file);
    let drain2 = slog_term::FullFormat::new(decorator).build().fuse();
    let drain2 = slog_async::Async::new(drain2).build().fuse();

    // Multiple drains.
    let drain = slog::Duplicate::new(drain1, drain2).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let root_logger = slog::Logger::root(drain, slog::o!());

    info!(root_logger, "--- Start module: {}", module_path!());
    info!(root_logger, "Module started"; "started_at" => format!("{}", Utc::now()));
    info!(root_logger, "Module ended"; "ended_at" => format!("{}", Utc::now()));
    info!(root_logger, "--- End module: {}", module_path!());
}