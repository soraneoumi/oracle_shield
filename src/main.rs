mod memory;
mod args;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use args::parse_args;

fn main() {
    let args = parse_args();
    let _buffers = memory::memory(args.memory);

    // Keep the program running until Ctrl-C is pressed
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
}
