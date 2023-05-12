use std::thread;
use std::time::Duration;

pub fn calculate_pi() {
    let handle = thread::spawn(|| {
        loop {
        }
    });
    thread::sleep(Duration::from_secs(300));
    handle.join().unwrap();
}
