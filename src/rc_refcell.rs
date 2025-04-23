use std::rc::Rc;
use std::cell::RefCell;
pub fn rc_refcell() {
    // Rc and RefCell
    // Rc (Reference Counted) is a smart pointer that enables multiple ownership of data.
    // RefCell is a mutable memory location with dynamically checked borrow rules.
    // Together, they allow for interior mutability, meaning you can mutate data even when it is behind an immutable reference.

let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));
let a = Rc::clone(&shared_vec);
let b = Rc::clone(&shared_vec);

a.borrow_mut().push(4);
a.borrow_mut().remove(3);
b.borrow_mut().remove(3); // Mutate through one owner
println!("{:?}", b.borrow()); // See the change through another owner: [1, 2, 3, 4]

}