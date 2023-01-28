// use std::sync::Arc;

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;

pub struct Logger;

impl Logger {
    pub fn start(level: LevelFilter) {
        // let file = std::fs::File::create("./log").unwrap();
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_thread_ids(true)
                    // .with_writer(Arc::new(file))
                    .with_filter(level)
            ).init()
    }
}
