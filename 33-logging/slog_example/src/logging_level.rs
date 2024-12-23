use chrono::Utc;
use slog::{self, debug, info, trace, Drain};
use std::sync::{atomic, Arc};

struct RuntimeLevelFilter<D> {
    drain: D,
    on: Arc<atomic::AtomicBool>,
}

impl<D> Drain for RuntimeLevelFilter<D>
where
    D: Drain,
{
    type Ok = Option<D::Ok>;
    type Err = Option<D::Err>;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> std::result::Result<Self::Ok, Self::Err> {
        let current_level = if self.on.load(atomic::Ordering::Relaxed) {
            // Enable `max_level_trace` feature of slog.
            // Slog by default removes
            // 1. trace and debug level statements in release builds,
            // 2. and trace level records in debug builds.
            slog::Level::Trace
        } else {
            slog::Level::Info
        };

        if record.level().is_at_least(current_level) {
            self.drain.log(record, values).map(Some).map_err(Some)
        } else {
            Ok(None)
        }
    }
}

pub fn test() {
    let on = Arc::new(atomic::AtomicBool::new(false));

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build();
    let drain = RuntimeLevelFilter {
        drain: drain,
        on: on.clone(),
    }
    .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let root_logger = slog::Logger::root(drain, slog::o!());

    // Switch to Debug log level.
    on.store(true, atomic::Ordering::Relaxed);

    info!(root_logger, "--- Start module: {}", module_path!());
    trace!(root_logger, "Module started"; "started_at" => format!("{}", Utc::now()));
    debug!(root_logger, "Module ended"; "ended_at" => format!("{}", Utc::now()));
    info!(root_logger, "--- End module: {}", module_path!());
}
