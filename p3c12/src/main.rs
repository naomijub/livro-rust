use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

#[derive(Debug)]
struct MyPi {
    pi: f32
}

impl MyPi {
    fn new() -> Self {
        Self {pi: 3.1415f32}
    }
}

fn my_pi() {
    let (tx, rx): (Sender<MyPi>, Receiver<MyPi>) = channel();
    thread::spawn(move || {
        tx.send(MyPi::new()).unwrap();
    });
    let pi = rx.recv().unwrap();
    println!("Pi is {:?}", pi);
}

use std::sync::mpsc::sync_channel;
use std::time::Duration;

fn sync() {
    let (tx, rx) = sync_channel(1);
    tx.send("Kakaroto!!").unwrap();
    thread::spawn(move || tx.send("Sou mais forte que você!").unwrap());

    thread::sleep(Duration::new(2, 0));

    if let Some(msg) = rx.recv().ok() {
      println!("Vegita: {:?}", msg);
    }
    if let Some(msg) = rx.recv().ok() {
      println!("Vegita: {:?}", msg);
    }
}

fn main() {
    // my_pi();
    sync();
}