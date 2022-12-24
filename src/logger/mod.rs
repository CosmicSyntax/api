use tracing::Level;

pub struct Logger;

impl Logger {
    pub fn start(level: Level) {
        tracing_subscriber::FmtSubscriber::builder()
            .with_thread_ids(true)
            .with_max_level(level)
            .init();
    }
}
