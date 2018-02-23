use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation : T,
    result : Option<u32>,
}

impl<T> Cacher<T>
    where T : Fn(u32) -> u32
{

    fn new(calculation : T) -> Cacher<T>
    {
        Cacher {
            calculation : calculation,
            result : None,
        }
    }

    fn value(&mut self, arg : u32) -> u32
    {
        match self.result {
            Some(i) => return i,
            None    => {
                let v = (self.calculation)(arg);
                self.result = Some(v);
                return v;
            },
        }
    }

}

fn some_long_running_task(num : u32) -> u32
{
    println!("started the task");
    thread::sleep(Duration::from_secs(2));
    println!("done with the task");
    return num;
}

fn generate_workout(intensity : u32, random_number : u32)
{
    let mut cacher = Cacher::new(
        |num| {
            return some_long_running_task(num);
        }
    );

    if intensity < 25 {
        println!("Do {} push ups", cacher.value(intensity));
        println!("Do {} sit ups", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("You have an off day today")
        } else {
            println!("Run for {} minutes", cacher.value(intensity));
        }
    }
}

fn main() {
    generate_workout(10, 12);
}
