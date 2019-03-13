use std::collections::HashMap;

fn main() {
    let mut d: HashMap<String, Vec<u64>> = HashMap::new();
    d.entry("a".to_string()).or_default().push(1);
    d.entry("a".to_string()).or_default().push(2);
    d.entry("a".to_string()).or_default().push(3);

    println!("{:?}",d );
}
