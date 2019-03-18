use regex::Regex;

fn main() {
    let line = "asdf fjdk; afed, fjek,asdf,    foo";

    println!("{:?}", Regex::new(r"[;,\s]\s*").unwrap().split(line).collect::<Vec<_>>());
}
