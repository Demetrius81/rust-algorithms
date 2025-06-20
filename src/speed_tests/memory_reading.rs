#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::time::Instant;

const NUM_ITERS: usize = 10_000;

pub fn run() {
    let mut arr = Box::new([[0; NUM_ITERS]; NUM_ITERS]);
    let mut now: Instant;

    println!("> Data from memory is read sequentially with minimal memory access.");
    now = std::time::Instant::now();
    for i in 0..NUM_ITERS {
        for j in 0..NUM_ITERS {
            arr[i][j] += 1;
        }
    }
    println!(">> Done in: {:?}", now.elapsed());

    println!("> Data from memory is not read sequentially. Each access to data is a cache miss.");
    now = std::time::Instant::now();
    for i in 0..NUM_ITERS {
        for j in 0..NUM_ITERS {
            arr[j][i] += 1;
        }
    }
    println!(">> Done in: {:?}", now.elapsed());
}
