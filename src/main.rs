use std::sync::{Arc, Mutex};

mod treadpool;
mod atomic_usize;
mod btree_map;
mod fn_once;
fn main(){

    use std::thread;

    let data = Arc::new(Mutex::new(vec![1,2,3]));

    let mut handles = Vec::new();

    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(4);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data: {:?}", *data.lock().unwrap());

    treadpool::tread_pool();
    atomic_usize::atomic_size();
    btree_map::btree_map();
    fn_once::fn_once();

}