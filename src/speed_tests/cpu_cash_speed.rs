use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use std::thread;

const NUM_THREADS: usize = 4;
const NUM_ITERS: usize = 100_000_000;

// #[repr(align(128))] // add paddind to 128 bytes (cache L1) to boost
#[derive(Debug, Default)]
struct Counter {
    pub val: AtomicUsize, // 8 bytes
}

#[derive(Debug, Default)]
struct Foo {
    val1: Counter,
    val2: Counter,
    val3: Counter,
    val4: Counter,
}

fn process() {
    let d = Arc::new(Foo::default());

    let mut handlers = vec![];

    for i in 0..NUM_THREADS {
        let d = d.clone();
        let h = thread::spawn(move || {
            for _ in 0..NUM_ITERS {
                match i % 4 {
                    0 => d.val1.val.fetch_add(1, Ordering::Relaxed),
                    1 => d.val2.val.fetch_add(1, Ordering::Relaxed),
                    2 => d.val3.val.fetch_add(1, Ordering::Relaxed),
                    _ => d.val4.val.fetch_add(1, Ordering::Relaxed),
                };
            }
        });

        handlers.push(h);
    }

    let now = std::time::Instant::now();

    for h in handlers {
        h.join().unwrap();
    }

    println!("> Done in:{:?}", now.elapsed());
    println!(">> Counter 1: {}", d.val1.val.load(Ordering::Relaxed));
    println!(">> Counter 2: {}", d.val2.val.load(Ordering::Relaxed));
    println!(">> Counter 3: {}", d.val3.val.load(Ordering::Relaxed));
    println!(">> Counter 4: {}", d.val4.val.load(Ordering::Relaxed));
}

pub fn run() {
    process();
}
