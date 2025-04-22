fn main(){
    let mut table = Vec::new();

    table.push(3);
    table.push(4);
    table.push(7);
    table.push(6);

    print!("{:?} ", table);
    table.sort();
    print!("{:?} ", table);
}