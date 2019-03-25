use regex;

fn main() {
    let tokens = [
        ("NAME", r#"[a-zA-Z_][a-zA-Z_0-9]*"#),
        ("NUM", r#"\d+"#),
        ("PLUS", r#"\+"#),
        ("TIMES", r#"\*"#),
        ("EQ", r#"="#),
        ("WS", r#"\s+"#)
    ];
    let text = "foo = 23 + 42 * 10";

    // Method 1: use RegexSet
    {
        let set = regex::RegexSet::new(
            tokens.iter().map(|(_name, pat)|["^", pat].join(""))
        ).unwrap();

        let mut i :usize = 0;
        loop {
            let m = set.matches(&text[i..]);
            if let Some(pat_idx) = m.iter().next() {
                let pat = tokens[pat_idx].1;
                // Unfortunately RegexSet only tells which pat is matched but not the matched text
                let m = regex::Regex::new(pat).unwrap().find_at(&text[i..], 0).unwrap();
                i += m.end();
                println!("{} {}", tokens[pat_idx].0, m.as_str());
            } else {
                break;
            }
        }
    }

    // Method 2: build a regex
    println!("-------------");
    {
        let r = regex::Regex::new(&tokens.iter().map(|(name, pat)|format!("(?P<{}>^{})", name, pat)).collect::<Vec<_>>().join("|")).unwrap();

        let mut i :usize = 0;
        loop {
            if let Some(captures) = r.captures(&text[i..]) {
                // Regex doesn't tell which group is captured, must enumerate.
                let (m, name) = tokens.iter().find_map(|(name,_pat)|captures.name(name).map(|m|(m, name))).unwrap();
                println!("{} {}", name, m.as_str());
                i += m.end();
                // Alternatively use captures.iter().skip(1)

            } else {
                break;
            }
        }
    }
}
