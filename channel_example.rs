use std::sync::mpsc::channel;
use std::thread::Thread;
use std::num::Float;
fn main() {
    let (tx, rx) = channel();
    let val = 0.4;
    Thread::spawn(move || {
        tx.send(val.sqrt()).unwrap();
    });
    println!("{:?}", rx.recv().unwrap());
}
