extern crate rustrees;

use rustrees::BinaryHeap;

#[allow(dead_code)]
fn main() {
    let xs = vec![96, 77, 88, 45, 53, 30, 78, 23, 36, 47];
    let mut hx = BinaryHeap::<i32>::from_vec(xs);
    println!("{:?}", hx);

    println!("{:?}", hx.pop());
    println!("{:?}", hx.pop());
    println!("{:?}", hx.pop());

    println!("{:?}", hx);

    let ys = vec![97, 84, 58, 73, 80, 30, 40, 49, 61, 66];
    let mut hy = BinaryHeap::<i32>::from_vec(ys);
    println!("{:?}", hy);

    hy.push(51);
    hy.push(32);
    hy.push(22);

    println!("{:?}", hy);
}
