use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    ///
    fn new(x : T) ->MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Deref coercion example
fn say_hello(name : &str)
{
    println!("Hello, {}", name);
}

fn main() {
    let b = Box::new(5);
    println!("b is {}", b);

    let x = 5;
    let b = MyBox::new(x);
    println!("b deref is {}", *b);

    let name = MyBox::new(String::from("Arun"));
    say_hello(&name);
}
