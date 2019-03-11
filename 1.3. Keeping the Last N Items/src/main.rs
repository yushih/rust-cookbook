use std::io::{BufRead, BufReader, Result};
use std::fs::File;
use std::collections::VecDeque;

struct HistoryIterator<I: Iterator> where I::Item: Clone {
    pub history: VecDeque<I::Item>,
    length: usize,
    iterator: I
}

impl<I: Iterator> HistoryIterator<I> where I::Item: Clone {
    pub fn new(iterator: I, length: usize) -> HistoryIterator<I> {
        HistoryIterator { history: VecDeque::new(), length, iterator}
    }

    pub fn get_history(&self) -> std::iter::Take<std::collections::vec_deque::Iter<I::Item>> {
        self.history.iter().take(self.history.len()-1)
    }
}

impl<I: Iterator> Iterator for HistoryIterator<I> where I::Item: Clone {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(i) => {
                self.history.push_back(i.clone());
                // must keep one more before the last item is the current item and not returned by get_history()
                if self.history.len() > self.length +1 {
                    self.history.pop_front();
                }
                return Some(i);
            },
            None => { None }
        }
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    if let Some(target) = args.next() {
        for arg in args {
            println!("Search for {} in {}:", target, arg);

            let mut iter =  HistoryIterator::new(BufReader::new(File::open(arg)?).lines().map(|l|l.unwrap()), 3);
            loop {
                if let Some(line) = iter.next() {
                    if line.contains(&target) {
                        for prefix in iter.get_history() {
                            println!("{}", prefix);
                        }
                        println!("{}", line);
                        println!("{}", "-".repeat(20));
                    }
                } else {
                    break;
                }
            }
        }
    } else {
        println!("No target.")
    }
    Ok(())
}
