use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));


    println!("counter after creating a = {}", Rc::strong_count(&a));

    let b: Rc<List> = Rc::new(Cons(3, Rc::clone(&a)));
    {
        let b: Rc<List> = Rc::new(Cons(3, Rc::clone(&a)));
        println!("counter after creating b = {}", Rc::strong_count(&a));
    }
    println!("counter after creating b = {}", Rc::strong_count(&a));
}
