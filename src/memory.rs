use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;
const GIB: usize = 1024 * MIB;

pub struct GiBObject {
    pub b: Arc<Mutex<Vec<u8>>>,
}

pub fn memory(gib: i32) -> Vec<GiBObject> {
    let mut buffers = Vec::with_capacity(gib as usize);
    let mut threads = Vec::new();

    for _ in 0..gib {
        let v = Arc::new(Mutex::new(vec![0; GIB]));
        let o = GiBObject { b: v.clone() };
        buffers.push(o);

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut v = v.lock().unwrap();
            for byte in &mut *v {
                *byte = rng.gen::<u8>();
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    buffers
}
