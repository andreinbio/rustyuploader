extern crate notify;

use notify::{Watcher, RecursiveMode, watcher, RecommendedWatcher, DebouncedEvent};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct Sentry {
    _watcher: RecommendedWatcher,
    channel_rx: Receiver<DebouncedEvent>,
}

impl Sentry {
    pub fn spy(path: &str) -> Self {
        // create a channel to receive the events
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events.
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        // Add the path to be watched.
        // All files and directories will be monitored
        watcher.watch(path, RecursiveMode::Recursive).unwrap();

        Sentry { _watcher: watcher, channel_rx: rx }
    }

    pub fn get_channel(&self) -> &Receiver<DebouncedEvent> {
        &self.channel_rx
    }
}
