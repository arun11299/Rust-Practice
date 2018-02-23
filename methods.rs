
struct Point
{
    x_ : f64,
    y_ : f64,
}

impl Point {

    fn origin() -> Point
    {
        Point { x_ : 0.0, y_ : 0.0 }
    }

    fn new(x : f64, y : f64) -> Point
    {
        Point { x_ : x, y_ : y }
    }

}

struct Rectangle
{
    p1_ : Point,
    p2_ : Point,
}

impl Rectangle {

    // &self is shorthand for writing self : &Self
    fn area(&self) -> f64
    {
        ((self.p1_.x_ - self.p2_.x_) * (self.p1_.y_ - self.p2_.y_)).abs()
    }

    fn perimeter(&self) -> f64
    {
        let Point { x_ : x1, y_ : y1 } = self.p1_;
        let Point { x_ : x2, y_ : y2 } = self.p2_;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x : f64, y : f64) -> ()
    {
        self.p1_.x_ += x;
        self.p2_.x_ += x;

        self.p1_.y_ += y;
        self.p2_.y_ += y;
    }

}

struct Pair(Box<i32>, Box<i32>);

impl Pair {

    fn destroy(self) -> ()
    {
        let Pair(first, second) = self;
        println!("Destroying pair ({}, {})", first, second);

        // first and second gets freed when it goes out of scope
    }

}

fn main() {
    let mut rect = Rectangle {
        p1_ : Point::origin(),
        p2_ : Point::new(1.0, 1.0),
    };

    println!("Area of rectangle = {}", rect.area());
    println!("Perimeter of rectangle = {}", rect.perimeter());

    rect.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

}
