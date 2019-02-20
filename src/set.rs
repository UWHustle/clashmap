use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::sync::{Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

const MIN_CAPACITY: usize = 2^4;

struct Bucket<T> {
    pub value: Mutex<Option<T>>
}

impl<T> Bucket<T> {
    pub fn new() -> Self {
        Bucket {
            value: Mutex::new(None)
        }
    }
}

pub struct ConcurrentHashSet<T> {
    hash_builder: RandomState,
    capacity: usize,
    size: AtomicUsize,
    buckets: Vec<Bucket<T>>
}

impl<T> ConcurrentHashSet<T> {
    pub fn new() -> Self {
        ConcurrentHashSet {
            hash_builder: RandomState::new(),
            capacity: MIN_CAPACITY,
            size: AtomicUsize::new(0),
            buckets: (0..MIN_CAPACITY).map(|_| Bucket::new()).collect()
        }
    }

    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn replace(&self, value: T) -> Option<T>
        where T: Hash + Eq
    {
        let hash = self.hash(&value);
        for i in 0..self.buckets.len() {
            let mut value_guard = self.buckets[(hash + i) % self.buckets.len()].value.lock().unwrap();
            if value_guard.is_none() || (*value_guard).as_ref().unwrap().eq(&value) {
                let existing_value = (*value_guard).replace(value);
                self.size += 1;
            }
        }
        None
    }

    fn hash(&self, value: &T) -> usize
        where T: Hash
    {
        let mut state = self.hash_builder.build_hasher();
        value.hash(&mut state);
        state.finish() as usize
    }
}
