fn  main(){

    let task1 = std::thread::spawn(|| {
        for i in 0..10 {
            println!("Task 1: {}", i);
        }
    });


    let task2 = {
        for i in 0..10 {
            println!("Task 2: {}", i);
        }
    };
    task1.join().unwrap();
    task2


}