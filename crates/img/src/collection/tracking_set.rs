use std::{
    collections::{BTreeSet, VecDeque},
    fmt::Debug,
    rc::Rc,
};

pub struct TrackingSet<T> {
    queue: VecDeque<Rc<T>>,
    tree: BTreeSet<Rc<T>>,
}

impl<T> Default for TrackingSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> TrackingSet<T> {
    pub fn new() -> Self {
        TrackingSet {
            queue: VecDeque::new(),
            tree: BTreeSet::new(),
        }
    }

    pub fn mid(&self) -> Option<&T> {
        let skip = self.tree.len() / 2;
        self.tree.iter().nth(skip).map(|v| &**v)
    }
}

impl<T: Ord> TrackingSet<T> {
    pub fn push(&mut self, value: T) {
        let value = Rc::new(value);
        self.queue.push_back(value.clone());
        self.tree.insert(value);
    }
}

impl<T: Ord + Debug> TrackingSet<T> {
    pub fn pop(&mut self) -> Option<T> {
        let value = self.queue.pop_front()?;
        self.tree.remove(&value);

        // SAFETY: rc lived only in both places
        Some(Rc::try_unwrap(value).unwrap())
    }
}
