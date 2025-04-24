pub fn iterating(){

    let mut  vec = vec![1, 2, 3, 4, 5];

    for i in vec.iter_mut() {
        println!("{}", i);
        *i = 10; // This will not compile because `i` is immutable
    }

    print!("vec: {:?}", vec);

}