
struct Point<T>
{
    x : T,
    y : T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct IntPoints
{
    x : i32,
    y : i32,
}

impl IntPoints {
    fn x(&self) -> &i32 {
        self.x
    }
}

fn main() {
    let p = Point{
        x : 5,
        y : 5,
    };

    let p2 = Point {
        x : 4f32,
        y : 4f32,
    };

    println!("Point is {}, {}", p.x(), p.y());

    println!("Distance from origin {}", p2.distance_from_origin());
}
