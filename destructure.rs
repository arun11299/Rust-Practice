
fn main() {
    let pair = (0, -2);
    println!("Pair is {:?}", pair);

    match pair {
        (0, y) => println!("First is 0, `y` is {:?}", y),
        (x, 0) => println!("`x` is {:?}, last is 0", x),
        _      => println!("couldnt care more"),
    }
}
