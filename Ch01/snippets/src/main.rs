use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let pointer = Rc::new(1);
    {
        let second_pointer = pointer.clone();
        println!("{}", *second_pointer);
    }
    // second_pointer is not here
    // println!("{}", *second_pointer);

    //Case 1: Mutable Shared pointer. (cant use in threads/ async)
    let shared_string = Rc::new(RefCell::new("Hello".to_string()));
    {
        let mut hello_world: RefMut<String> = shared_string.borrow_mut();
        hello_world.push_str(" World")
    }
    println!("{}", shared_string.take());

    // Case 2: Share across threads.
    let pointer = Arc::new(5);
    let second_pointer = pointer.clone();
    thread::spawn(move || {
        println!("{}", *second_pointer);
    });
    thread::sleep(time::Duration::from_secs(1));
    println!("{}", *pointer);

    // Case 3: Mutable, shared pointers.
    let pointer = Arc::new(Mutex::new(5));
    let second_pointer = pointer.clone();
    thread::spawn(move || {
        let mut mutable_pointer = second_pointer.lock().unwrap();
        *mutable_pointer = 1;
    });
    thread::sleep(time::Duration::from_secs(1));
    let one = pointer.lock().unwrap();
    println!("{}", one);
}
