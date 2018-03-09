struct Coordinate {
    x: i32,
    y: i32
}
 
fn get_x(c: &Coordinate) -> &i32 {
    &c.x
}
 
fn main() {
    let x;
 
    {
        let c = Coordinate { x: 1, y: 2 };
        x = get_x(&c);
    }
 
    println!("{}", x);
}
