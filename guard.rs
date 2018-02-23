
fn main() {
    let pair = (2, -2);

    match pair {
        (x, y) if x == y  => println!("exact match"),
        (x, y) if x == -y => println!("mirrored match"),
        _ => println!("got the idea")
    }
}
