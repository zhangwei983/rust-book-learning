use chrono::Utc;
use slog::{self, info, Drain};

pub fn test() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let root_logger = slog::Logger::root(drain, slog::o!());

    info!(root_logger, "--- Start module: {}", module_path!());
    info!(root_logger, "Module started"; "started_at" => format!("{}", Utc::now()));
    info!(root_logger, "Module ended"; "ended_at" => format!("{}", Utc::now()));
    info!(root_logger, "--- End module: {}", module_path!());
}
