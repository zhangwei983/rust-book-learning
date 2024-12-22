use chrono::Utc;
use slog::{self, debug, info, Drain};
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
            slog::Level::Debug
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

    info!(root_logger, "--- Start module: {}", module_path!());

    debug!(root_logger, "Module started";
        "started_at" => format!("{}", Utc::now()));

    // Enable Debug log level.
    on.store(true, atomic::Ordering::Relaxed);

    debug!(root_logger, "Module ended";
        "ended_at" => format!("{}", Utc::now()));

    info!(root_logger, "--- End module: {}", module_path!());
}
