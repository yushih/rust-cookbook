use std::collections::HashMap;

fn main() {
    let prices: HashMap<&str, f64> =
        [("ACME", 45.23),
         ("AAPL", 612.78),
         ("IBM", 205.55),
         ("HPQ", 37.20),
         ("FB", 10.75)]
        .iter().cloned().collect();

    let mut a : Vec<(f64, &str)> = prices.iter().map(|(s, p)|(p.clone(),s.clone())).collect();

  

    let min_price = a.iter().min_by(|(p1,_), (p2,_)|p1.partial_cmp(p2).unwrap()).cloned();
    let max_price = a.iter().max_by(|(p1,_), (p2,_)|p1.partial_cmp(p2).unwrap()).cloned();
    a.sort_by(|(p1,_), (p2,_)|p1.partial_cmp(p2).unwrap());

    println!("{:?}", min_price);
    println!("{:?}", max_price);
    println!("{:?}", a);
}
