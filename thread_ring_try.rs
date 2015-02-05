use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread::Thread;

fn main() {
    let n_threads: usize = 10;
    let t_space = 0..n_threads - 1;
    let channels: Vec<(Sender<i64>, Receiver<i64>)> = 
        t_space.map(|_| { channel() }).collect();
    let thread_channels: Vec<_> = t_space.map(|n| {
        (channels[n].0, channels[(n + 1) % n_threads].1, n)
    }).collect();

    let guards: Vec<_> = thread_channels.iter().map(|t| {
        Thread::scoped(move || {
            let &(ref tx, ref rx, n) = t;
            rx.recv();
            println!("Thread {} says hi!", n);
            tx.send(10);
        })
    }).collect();

    guards.iter().map(|x| { x.join() });
}
