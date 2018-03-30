extern crate vector;
use std::fmt::Debug;
use vector::Vector;

fn print<T>(v: &Vector<T>)
    where T : Debug
{
    for i in 0..v.size() {
        /// This works because we implemented Deref
        /// for the Vector. Doing that compiler
        /// coerces it into a slice to which indexing
        /// can be done (see std::ops::Index for [T])
        print!("{:?} ", v[i]);
    }
    println!();
}

fn main() {
    let mut v: Vector<i32> = Vector::new();
    v.push(1);
    v.push(2);
    v.push(3);

    print(&v);
    v.insert(1, 4);
    print(&v);
    v.remove(3);
    print(&v);

    for i in v.into_iter() {
        println!("{:?}", i);
    }
}
