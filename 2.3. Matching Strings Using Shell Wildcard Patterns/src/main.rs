use glob::Pattern;

fn main() {
    let names = ["Dat1.csv", "Dat2.csv", "config.ini", "foo.py"];
    let pat = Pattern::new("Dat*.csv").unwrap();

    println!("{:?}", names.iter().filter(|name|pat.matches(name)).collect::<Vec<_>>());
}
