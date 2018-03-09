extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
    // get a random number within the range 1 to 101
    let mut rng = thread_rng();
    let secret_number = rng.gen_range(0, 101);

    loop {
        let mut guess = String::new();

        println!("Enter your guess: ");

        io::stdin().read_line(&mut guess)
                   .expect("Failed to read line");

        /*
        let guess : u32 = guess.trim().parse()
                             .expect("Please type a number!");
        */
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Invalid input. Try again!");
                continue;
            },
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => println!("you win!!")
        }
    }
}
