use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    //Sending thread
    let handle1 = thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let recv_val = rx.recv().unwrap();
    println!("Received value is: {}", recv_val);
    handle1.join().unwrap();
}
