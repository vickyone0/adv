use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub fn tread_pool() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let pool = ThreadPool::new(4); // Create a thread pool with 4 threads

    for _ in 0..5 {
        let data = Arc::clone(&data);
        pool.execute(move || {
            let mut data = data.lock().unwrap();
            data.push(2);
        });
    }

    // Wait for all tasks to complete
    pool.join();

    println!("Final data: {:?}", *data.lock().unwrap());
}
