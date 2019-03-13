use indexmap::IndexMap;

fn main() {
    let mut d = IndexMap::new();

    d.insert("foo".to_string(), 1);
    d.insert("bar".to_string(), 2);
    d.insert("spam".to_string(), 3);
    d.insert("grok".to_string(), 4);

    for (k, v) in d {
        println!("{} {}", k, v);
    }
}
