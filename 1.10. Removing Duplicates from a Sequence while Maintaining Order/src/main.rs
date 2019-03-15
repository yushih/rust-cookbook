use std::rc::Rc;
use std::hash::Hash;
use std::collections::HashSet;

struct DedupIterator<'a, T> where T: Eq+Hash {
    iterator: &'a mut Iterator<Item=T> ,
    seen: HashSet<Rc<T>>
}


fn dedup<T>(iterator : &mut Iterator<Item=T>) -> DedupIterator<T> where T: Eq+Hash {
    DedupIterator {iterator,
                   seen: HashSet::new()}
}

impl<'a, T> Iterator for DedupIterator<'a, T> where T: Eq+Hash {
    type Item = Rc<T>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next) = self.iterator.next() {
                let p = Rc::new(next);
                if self.seen.insert(p.clone()) {
                    return Some(p);
                }
            } else {
                return None;
            }
        }
         
    }
}

fn main() {
    let a = [1, 5, 2, 1, 9, 1, 5, 10];
    
    println!("{:?}", dedup(&mut a.iter()).collect::<Vec<_>>());
}
