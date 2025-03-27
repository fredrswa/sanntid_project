// timer.rs
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Timer {
    timed_out: Arc<AtomicBool>,    // Flag to check if the timer has expired
    cancel_flag: Arc<AtomicBool>,  // Flag to cancel the current timer
    timeout: Duration,             // Duration after which the timer expires
}

impl Timer {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timed_out: Arc::new(AtomicBool::new(false)),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            timeout,
        }
    }

    /// Starts the timer. Cancels any existing timer.
    pub fn start(&self) {
        // Cancel any running timer
        self.cancel_flag.store(true, Ordering::Relaxed);

        // Reset flags for the new timer
        self.timed_out.store(false, Ordering::Relaxed);
        self.cancel_flag.store(false, Ordering::Relaxed);

        // Clone Arc values to move into the thread
        let timed_out = self.timed_out.clone();
        let cancel_flag = self.cancel_flag.clone();
        let timeout = self.timeout;

        // Spawn the timer thread
        thread::spawn(move || {
            let start = std::time::Instant::now();
            loop {
                // Exit if cancellation is requested
                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                // Check if the timeout has elapsed
                if start.elapsed() >= timeout {
                    println!("Timer expired");
                    timed_out.store(true, Ordering::Relaxed);
                    break;
                }

                // Sleep briefly to avoid busy-waiting
                thread::sleep(Duration::from_millis(25));
            }
        });
    }

    /// Returns `true` if the timer has expired.
    pub fn is_expired(&self) -> bool {
        self.timed_out.load(Ordering::Relaxed)
    }

    pub fn expired_used(&self) {
        self.timed_out.store(false, Ordering::Relaxed);
    }

    /// Cancels the current timer (if running).
    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
        self.timed_out.store(false, Ordering::Relaxed);
    }
}