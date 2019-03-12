use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::iter::from_fn;
use std::iter::FromIterator;

fn main() {
    let nums = vec![1, 8, 2, 23, 7, -4, 18, 23, 42, 37, 2];
    let mut max_heap = BinaryHeap::from_iter(nums.iter());
    println!("n largest {:?}", from_fn(||max_heap.pop()).take(3).collect::<Vec<_>>());

    let mut min_heap = BinaryHeap::from_iter(nums.iter().map(Reverse));
    println!("n smallest {:?}", from_fn(||min_heap.pop()).take(3).map(|r|r.0).collect::<Vec<_>>());

}
