
use std::rc::Rc;

enum List
{
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("Count after creating b = {}", Rc::strong_count(&a));
    println!("Count of b = {}", Rc::strong_count(&b));
}
