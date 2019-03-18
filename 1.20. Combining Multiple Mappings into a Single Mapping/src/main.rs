use std::hash::Hash;
use std::collections::HashMap;

fn make_chain_mapper<'a, K, V>(maps:Vec<&'a HashMap<K, V>>) -> Box<Fn(K)->Option<&'a V> + 'a> 
where K: Eq+Hash {
    Box::new(move |k| {
        for m in &maps {
            if let Some(v) = m.get(&k) {
                return Some(v);
            }
        }
        return None
    })
}

fn main() {
    let mut a = HashMap::new();
    a.insert("x", 1);
    a.insert("z", 3);

    let mut b = HashMap::new();
    b.insert("y", 2);
    b.insert("z", 4);

    let chain_mapper = make_chain_mapper(vec![&a, &b]);
    println!("{}", chain_mapper("x").unwrap());
    println!("{}", chain_mapper("y").unwrap());
    println!("{}", chain_mapper("z").unwrap());
}
//ref: https://users.rust-lang.org/t/writing-functions-that-take-and-return-closures/7743
