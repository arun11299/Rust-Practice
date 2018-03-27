use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point
{
    x : i32,
    y : i32,
}

impl Add for Point {
    /// Associated type Output
    type Output = Point;

    fn add(self, other : Point) -> Point
    {
        Point{
            x : (self.x + other.x),
            y : (self.y + other.y),
        }
    }
}

//----------------------------------------------

struct MilliMeters(u32);
struct Meters(u32);

impl Add for MilliMeters {
    type Output = MilliMeters;

    fn add(self, other : MilliMeters) -> MilliMeters
    {
        MilliMeters(self.0 + other.0)
    }
}

impl Add<Meters> for MilliMeters {
    type Output = MilliMeters;

    fn add(self, other : Meters) -> MilliMeters
    {
        MilliMeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let p1 = Point{x:1, y:2};
    let p2 = Point{x:3, y:4};

    let res = p1 + p2;
    assert!(res.x == 4);
    assert!(res.y == 6);
}
