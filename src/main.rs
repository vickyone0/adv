use std::sync::{Arc, Mutex};

mod atomic_usize;
mod btree_map;
mod builder;
//use builder::bicycle_builder::BicycleBuilder;
mod fn_once;
mod iter;
mod mutax_condver;
mod rc_refcell;
mod treadpool;
mod fluent;
fn main() {
    use std::thread;

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

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

    // treadpool::tread_pool();
    // atomic_usize::atomic_size();
    // btree_map::btree_map();
    // fn_once::fn_once();
    // rc_refcell::rc_refcell();
    iter::iterating();
    mutax_condver::mutex_condvar();


    let mut bicycle_builder = builder::BicycleBuilder::new();
    println!("My new bike: {:?}", bicycle_builder);
bicycle_builder.with_make("Huffy");
bicycle_builder.with_model("Radio");
bicycle_builder.with_size(46);
bicycle_builder.with_color("red");

let bicycle = bicycle_builder.build();
println!("My new bike: {:#?}", bicycle);
//let bicycle2 = bicycle_builder.build();
println!("My new bike: {:#?}", bicycle);
//bicycle_builder.bicycle.make();

fluent::out();
}
