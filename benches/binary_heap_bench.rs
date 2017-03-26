#[macro_use]
extern crate bencher;

extern crate data_structures;
extern crate rand;

use bencher::Bencher;
use data_structures::binary_heap::BinaryHeap;
use rand::Rng;

fn bench_push_1k(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}

fn bench_push_10k(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..10000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}

fn bench_push_100k(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..100000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}

fn bench_push_1m(b: &mut Bencher) {
    let mut heap = BinaryHeap::<u32>::new();
    let mut rng = rand::thread_rng();

    for _ in 0..1000000 {
        heap.push(rng.gen());
    }

    b.iter(|| {
        heap.push(rng.gen());
    });
}

benchmark_group!(benches, bench_push_1k, bench_push_10k, bench_push_100k, bench_push_1m);
benchmark_main!(benches);
