use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn set_signal_handler(running: Arc<AtomicBool>) {
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
}
