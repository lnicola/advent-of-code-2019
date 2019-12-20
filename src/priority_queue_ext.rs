use priority_queue::PriorityQueue;
use std::hash::{BuildHasher, Hash};

pub trait PriorityQueueExt<I, P, H>
where
    I: Hash + Eq,
    P: Ord + Clone,
    H: BuildHasher,
{
    fn push_increase(&mut self, item: I, priority: P);
}

impl<I, P, H> PriorityQueueExt<I, P, H> for PriorityQueue<I, P, H>
where
    I: Hash + Eq,
    P: Ord + Clone,
    H: BuildHasher,
{
    fn push_increase(&mut self, item: I, priority: P) {
        let mut found = false;
        let p = priority.clone();
        self.change_priority_by(&item, |old_p| {
            found = true;
            p.max(old_p)
        });
        if !found {
            self.push(item, priority);
        }
    }
}
