use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use std::thread;
pub fn atomic_size() {
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    println!("Final data: {:?}", handles[0]);
}
