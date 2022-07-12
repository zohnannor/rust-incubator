use std::thread;

use crossbeam::channel::{self, Receiver, Sender};
use rand::RngCore;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Matrix = [[u8; 64]; 64];

struct Producer;

impl Producer {
    fn spawn(tx: Sender<Matrix>) -> Self {
        thread::Builder::new()
            .name("Producer".into())
            .spawn(move || loop {
                let m: Matrix = {
                    let mut m = [[0u8; 64]; 64];
                    for row in &mut m {
                        rand::thread_rng().fill_bytes(row);
                    }
                    m
                };
                tx.send(m).unwrap();
            })
            .unwrap();

        Self
    }
}

struct Consumer;

impl Consumer {
    fn spawn(rx: Receiver<Matrix>) -> Self {
        thread::Builder::new()
            .name("Consumer".into())
            .spawn(move || loop {
                let m: Matrix = rx.recv().unwrap();

                let sum: u32 = m.into_par_iter().flatten_iter().map(u32::from).sum();

                println!(
                    "{}({:?}) computed sum: {:?}",
                    thread::current().name().unwrap(),
                    thread::current().id(),
                    sum
                );
            })
            .unwrap();

        Self
    }
}

fn main() {
    let (tx, rx) = channel::unbounded();

    let _producer = Producer::spawn(tx.clone());
    let _consumer1 = Consumer::spawn(rx.clone());
    let _consumer2 = Consumer::spawn(rx.clone());

    loop {
        let matrix = rx.recv().unwrap();
        tx.send(matrix).unwrap();
    }
}
