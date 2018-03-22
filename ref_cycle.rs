use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
use List::{Cons, Nil};
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>>
    {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil               => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("Initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a ref count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b ref count after changing a = {}", Rc::strong_count(&b));
    println!("a ref count after changing a = {}", Rc::strong_count(&a));

    // Uncommenting below line will result in stack overflow!
    //println!("a next item = {:?}", a.tail());
}
