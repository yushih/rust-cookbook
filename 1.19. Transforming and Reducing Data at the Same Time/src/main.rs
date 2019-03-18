#[macro_use(c)]
extern crate cute;

fn main() {
    let nums  = [1, 2, 3, 4, 5];
    let s : i32 = c![x*x, for x in &nums].iter().sum();
    println!("s={}", s);
}
