#![feature(test)]

extern crate data_structures;
extern crate test;
extern crate rand;

use test::Bencher;
use data_structures::binary_heap::BinaryHeap;
use rand::Rng;

#[bench]
fn bench_push_1000(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}

#[bench]
fn bench_push_10000(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..10000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}

#[bench]
fn bench_push_100000(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..100000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}

#[bench]
fn bench_push_1000000(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..1000000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}
