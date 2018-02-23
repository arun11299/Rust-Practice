
fn main() {
    let number = Some(69);
    let op1 : Option<i32> = None;
    let op2 : Option<i32> = None;

    if let Some(i) = number {
        println!("wrapped number is {}", i);
    } else {
        println!("there is nothing")
    }
}
