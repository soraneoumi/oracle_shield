use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;

const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;
const GIB: usize = 1024 * MIB;

pub struct GiBObject {
    pub b: Arc<RwLock<Vec<u8>>>,
}

pub fn memory(gib: i32) -> Vec<GiBObject> {
    let mut buffers = Vec::with_capacity(gib as usize);

    for _ in 0..gib {
        let v = Arc::new(RwLock::new(vec![0; GIB]));
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
