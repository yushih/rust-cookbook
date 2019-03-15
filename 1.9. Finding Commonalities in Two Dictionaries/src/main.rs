use std::collections::HashSet;

fn main() {
    let k1 = ["x", "y", "z"];
    
    let k2 = ["w", "x", "y"];

    println!("{:?}", k1.iter().collect::<HashSet<_>>().intersection(&k2.iter().collect::<HashSet<_>>()));

    println!("{:?}", k1.iter().collect::<HashSet<_>>().difference(&k2.iter().collect::<HashSet<_>>()));
}
