use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let m = Arc::new(Mutex::new(0));
    let m1 = Arc::clone(&m);
    let m2 = Arc::clone(&m1);

    let handle1 = thread::spawn(move || {
        loop {
            let mut num = m1.lock().unwrap();
            *num = *num + 1;
            println!("Thread-1 value: {}", *num);
            if *num > 100 {
                break;
            }
        }
    });

    let handle2 = thread::spawn(move || {
        loop {
            let mut num = m2.lock().unwrap();
            *num = *num + 1;
            println!("Thread-2 value: {}", *num);
            if *num > 100 {
                break;
            }
        }
    });

    println!("Shared value: {}", *m.lock().unwrap());

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Shared value: {}", *m.lock().unwrap());
}
