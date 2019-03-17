use std::collections::HashMap;

fn main() {
    let words = [
        "look", "into", "my", "eyes", "look", "into", "my", "eyes",
        "the", "eyes", "the", "eyes", "the", "eyes", "not", "around", "the",
        "eyes", "don\"t", "look", "around", "the", "eyes", "look", "into",
        "my", "eyes", "you\"re", "under"];

    let mut word_count = HashMap::new();

    for w in &words {
        word_count.entry(w).and_modify(|c|*c+=1).or_insert(1);
    }

    let top3 = {
        let mut freq = word_count.iter().collect::<Vec<_>>();

        freq.sort_by_key(|(_w,c)|c.clone());

        freq.iter().rev().take(3).cloned().collect::<Vec<_>>()
    };

    println!("{:?}", top3);
}
