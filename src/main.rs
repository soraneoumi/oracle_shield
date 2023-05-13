mod args;
mod cpu;
mod memory;
mod signal;

use args::{parse_and_validate_args, Args};
use signal::set_signal_handler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let Args { memory, frequency } = parse_and_validate_args();
    let buffers = memory.map(memory::memory);
    let interval = frequency
        .map(|f| Duration::from_secs(30 * 24 * 60 * 60 / f as u64))
        .unwrap_or(Duration::from_secs(u64::MAX));
    let running = Arc::new(AtomicBool::new(true));
    set_signal_handler(running.clone()).await;
    let mut last_calculation = Instant::now();
    let mut calculation_start_time = None;
    while running.load(Ordering::SeqCst) {
        if let Some(buffers) = &buffers {
            for buffer in buffers {
                let _ = buffer.b.read().unwrap()[0];
            }
        }

        if last_calculation.elapsed() >= interval {
            calculation_start_time = Some(Instant::now());
            last_calculation = Instant::now();
        }

        if let Some(start_time) = calculation_start_time {
            if start_time.elapsed() < Duration::from_secs(5 * 60) {
                cpu::calculate_pi(Duration::from_secs(5 * 60)).await;
            } else {
                calculation_start_time = None;
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
}
