
fn main() {
    for n in 1..20 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            continue;
        }
    }

    let names = vec!["bob", "frank", "arun"];

    for name in names.iter() {
        match name {
            &"arun" => println!("Arun is a rustacean"),
            _       => println!("Hello {}", name),
        }
    }

    println!("First name is {}", names[0]);

    for name in names.into_iter() {
        match name {
            "arun" => println!("Arun is a ruatacean"),
            _      => println!("Hello {}", name),
        }
    }

    // Destructive mobe in rust
    //println!("First name is {}", names[0]);
}
