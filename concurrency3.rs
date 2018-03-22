use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("This"),
            String::from("is"),
            String::from("message"),
            String::from("from"),
            String::from("thread-1"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("This"),
            String::from("is"),
            String::from("message"),
            String::from("from"),
            String::from("thread-2"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("Message Received: {}", msg);
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}
