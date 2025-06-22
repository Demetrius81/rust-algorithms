#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::time::{Duration, Instant};

const NUM_ITERS: usize = 100_000_000;

#[derive(Clone, Debug)]
#[repr(Rust)]
struct Foo {
    a: u8,
    b: u64,
    c: u16,
}

#[derive(Clone, Debug)]
#[repr(C)]
struct Bar {
    a: u8,
    b: u64,
    c: u16,
}

#[derive(Clone, Debug)]
#[repr(C)]
struct BarP {
    b: u64,
    c: u16,
    a: u8,
}

pub fn run() {
    let foo_vec = vec![Foo { a: 1, b: 2, c: 3 }; NUM_ITERS];
    let bar_vec = vec![Bar { a: 1, b: 2, c: 3 }; NUM_ITERS];
    let barp_vec = vec![BarP { a: 1, b: 2, c: 3 }; NUM_ITERS];
    let mut now: Instant;
    let mut packed_time: Duration;
    let mut sum: u64;

    crate::speed_tests::memory_representation::print_memory_representation(foo_vec[0].clone());
    crate::speed_tests::memory_representation::print_memory_representation(bar_vec[0].clone());
    crate::speed_tests::memory_representation::print_memory_representation(barp_vec[0].clone());

    now = std::time::Instant::now();
    sum = foo_vec.iter().map(|x| x.b).sum();
    packed_time = now.elapsed();
    println!("> Sum:{}, Done in:{:?}", sum, packed_time);

    now = std::time::Instant::now();
    sum = bar_vec.iter().map(|x| x.b).sum();
    packed_time = now.elapsed();
    println!("> Sum:{}, Done in:{:?}", sum, packed_time);

    now = std::time::Instant::now();
    sum = barp_vec.iter().map(|x| x.b).sum();
    packed_time = now.elapsed();
    println!("> Sum:{}, Done in:{:?}", sum, packed_time);
}
