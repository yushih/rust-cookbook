fn main() {
    // There is no equivalent of Python expression "first, *middle, last = list" in Rust
    let array = [1,2,3,4,5];
    let first = array.first();
    let middle = &array[1..array.len()-1];
    let last = array.last();
    println!("first={:?} middle={:?} last={:?}", first, middle, last);
}
