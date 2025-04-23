

pub fn fn_once() {
    // fn once
    // A function that takes ownership of its arguments and can only be called once.
    // It is useful for creating closures that capture variables by value.
    // The `move` keyword is used to indicate that the closure takes ownership of the captured variables.
    
    // Example:
let consumable = String::from("cookie");
let consumer = move || consumable;
consumer();
//consumer();
}