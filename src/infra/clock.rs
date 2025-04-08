//! Clock Utility
//!
//! Provides shared timing, tick scheduling, and block simulation.

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

pub struct Clock;

impl Clock {
    /// Returns current UNIX timestamp in seconds.
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs()
    }

    /// Sleeps for the given number of milliseconds.
    pub async fn sleep_ms(ms: u64) {
        sleep(Duration::from_millis(ms)).await;
    }

    /// Simple ticking loop. Runs a callback every `interval_ms`.
    pub async fn start_loop<F>(interval_ms: u64, mut callback: F)
    where
        F: FnMut(u64) + Send + 'static,
    {
        loop {
            let now = Clock::now();
            callback(now);
            Clock::sleep_ms(interval_ms).await;
        }
    }
}
