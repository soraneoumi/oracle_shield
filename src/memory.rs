use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::SystemTime;
use chrono::{DateTime, Local};

const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;
const GIB: usize = 1024 * MIB;

pub struct GiBObject {
    pub b: Arc<RwLock<Vec<u8>>>,
}

pub fn memory(gib: i32) -> Vec<GiBObject> {
    let now: DateTime<Local> = SystemTime::now().into();
    println!("{} Started using memory", now.format("%Y-%m-%d %H:%M:%S").to_string());

    let mut buffers = Vec::with_capacity(gib as usize);

    for _ in 0..gib {
        let v = Arc::new(RwLock::new(Vec::<u8>::with_capacity(GIB)));
        v.write().unwrap().resize(GIB, 0);
        let o = GiBObject { b: v.clone() };
        buffers.push(o);
    }

    buffers.par_iter().for_each(|o| {
        let mut rng = rand::thread_rng();
        let mut v = o.b.write().unwrap();
        for byte in &mut *v {
            *byte = rng.gen::<u8>();
        }
    });

    buffers
}
