mod memory;

use std::env;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} -m <memory in GiB>", args[0]);
        process::exit(1);
    }
    let memory_arg = args[2].parse::<i32>().unwrap_or_else(|err| {
        eprintln!("Error parsing memory argument: {}", err);
        process::exit(1);
    });
    let _buffers = memory::memory(memory_arg);

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
