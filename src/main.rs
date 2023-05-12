mod memory;
mod args;
mod cpu;
mod signal;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use args::{parse_and_validate_args, Args};
use signal::set_signal_handler;

fn main() {
    let Args { memory, frequency } = parse_and_validate_args();
    let buffers = if let Some(memory) = memory {
        Some(memory::memory(memory))
    } else {
        None
    };
    let interval = if let Some(frequency) = frequency {
        Duration::from_secs(30 * 24 * 60 * 60 / frequency as u64)
    } else {
        Duration::from_secs(u64::MAX)
    };
    let running = Arc::new(AtomicBool::new(true));
    set_signal_handler(running.clone());
    let mut last_calculation = Instant::now();
    while running.load(Ordering::SeqCst) {
        if let Some(buffers) = &buffers {
            for buffer in buffers {
                let _ = buffer.b.read().unwrap()[0];
            }
        }

        if last_calculation.elapsed() >= interval {
            cpu::calculate_pi();
            last_calculation = Instant::now();
        }
        thread::sleep(Duration::from_millis(100));
    }
}
