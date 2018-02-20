//use std::convert::Format;

struct Number
{
    num_ : i32,
}

impl From<i32> for Number {

    fn from(item : i32) -> Self {
        Number{ num_ : item }
    }

}

fn main() {
    let n : i32 = 42;
    let numb = Number::from(n);
    println!("Num is {}", numb.num_);

    let numb2 : Number = n.into();

    println!("--------------------");

    // For custom types FromStr trait needs to be implemented
    let parsed : i32 = "5".parse().unwrap();
    // Parsing with type inference
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Result is {}", sum);
}
