mod lib;

use lib::*;
use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn hello(name: &str) {
    println!("hello {}!", name)
}

fn main() {
    my_box_test();

    custom_smart_pointer_test();

    reference_counting_test();
}

fn my_box_test() {
    let b = Box::new(5);
    println!(" b = {}", b);
    let my_box = lib::MyBox::new("Rust");
    hello(&my_box);
}

fn custom_smart_pointer_test() {
    let pointer = CustomSmartPointer {
        data: String::from("My stuff")
    };
    let pointer_2 = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("CustomSmartPointer created");

    drop(pointer);
}


fn reference_counting_test() {
    let reference_counting = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&reference_counting));

    let reference_counting_copy = Cons(3, Rc::clone(&reference_counting));




    println!("count after creating b = {}", Rc::strong_count(&reference_counting));
    {
        let reference_counting_secondo_copy = Cons(4, Rc::clone(&reference_counting));
        println!("count after creating c = {}", Rc::strong_count(&reference_counting))
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&reference_counting))
}
