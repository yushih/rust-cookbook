use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct PriorityQueueItem<T> {
    priority: usize,
    index: usize,
    pub item: T
}

impl<T> PriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.priority == other.priority {
            return other.index.cmp(&self.index);
        } else {
            return self.priority.cmp(&other.priority);
        }
    }
}

impl<T> PartialEq for PriorityQueueItem<T> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<T> Eq for PriorityQueueItem<T> {}

impl<T> PartialOrd for PriorityQueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for PriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

//#[derive(Default)]
struct PriorityQueue<T> {
    queue: BinaryHeap<PriorityQueueItem<T>>,
    index: usize
}

impl<T> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue{queue: BinaryHeap::new(), index: 0}
    }

    pub fn push(&mut self, item: T, priority: usize) -> &mut Self {
        self.queue.push(PriorityQueueItem{priority, index: self.index, item});
        self.index += 1;
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop().map(|i|i.item)
    }

}

fn main() {
    let mut q = PriorityQueue::new();

    q.push("foo".to_string(), 1);
    q.push("bar".to_string(), 5);
    q.push("spam".to_string(), 4);
    q.push("grok".to_string(), 1);

    loop {
        if let Some(i) = q.pop() {
            println!("{}", i);
        } else {
            break;
        }
    }
}
