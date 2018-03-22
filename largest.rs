
use std::cmp::PartialOrd;

fn largest(list : &[i32]) -> i32
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn gen_largest<T : PartialOrd>(list : &[T]) -> T
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn main() {
    let list = vec![5,32,7,8,3,23,6,4,2];
    let mut result = largest(&list);
    println!("Largest is {}", result);

    result = gen_largest(&list);
    println!("Largest is {}", result);

    let a = String::from("test string");
    let ref b = a;
    let c = b;
    println!("C is {}", c);
    println!("A is {}", a);
    println!("B is {}", b);
}
