use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> u64;
}

#[derive(Debug)]
struct Rectangle
{
    length : u64,
    height : u64,
}

impl HasArea for Rectangle {

    fn area(&self) -> u64 {
        self.length * self.height
    }

}

fn print_area<T : HasArea + Debug>(t : &T)
{
    println!("Area is of {:?} is = {}", t, t.area());
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn only_blues<T : Blue>(t : &T) -> &'static str { "blue" }
fn only_reds<T : Red>(t : &T) -> &'static str { "red" }

fn main() {
    let rect = Rectangle { length: 2, height: 2};
    print_area(&rect);

    println!("{}", only_blues(&BlueJay))
}
