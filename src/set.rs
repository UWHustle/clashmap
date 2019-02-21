use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::sync::{Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::MutexGuard;

const MIN_CAPACITY: usize = 2^4;
const MAX_LOAD_NUM: usize = 1;
const MAX_LOAD_DEN: usize = 2;

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
    size: AtomicUsize,
    raw_capacity: AtomicUsize,
    buckets: RwLock<Vec<Bucket<T>>>
}

impl<T> ConcurrentHashSet<T> {
    pub fn new() -> Self {
        ConcurrentHashSet {
            hash_builder: RandomState::new(),
            size: AtomicUsize::new(0),
            raw_capacity: AtomicUsize::new(MIN_CAPACITY),
            buckets: RwLock::new((0..MIN_CAPACITY).map(|_| Bucket::new()).collect())
        }
    }

    pub fn capacity(&self) -> usize {
        self.raw_capacity.load(Ordering::Relaxed) * MAX_LOAD_NUM / MAX_LOAD_DEN
    }

    pub fn reserve(&self, additional: usize) {
        let remaining = self.capacity() - self.len();
        if remaining < additional {
            let min_capacity = self.len() + additional;
            let raw_capacity = MAX_LOAD_DEN * min_capacity / MAX_LOAD_NUM;
        }
    }

    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn replace(&self, value: T) -> Option<T>
        where T: Hash + Eq
    {
        let hash = self.hash(&value);
        self.reserve(1);

        let buckets_guard = self.buckets.read().unwrap();
        let mut i = hash % buckets_guard.len();

        let mut value_guard = loop {
            let value_guard = buckets_guard[i].value.lock().unwrap();
            if (*value_guard).is_none() || (*value_guard).as_ref().unwrap() == &value {
                break value_guard;
            }
            i = (i + 1) % buckets_guard.len();
        };

        let replaced_value = (*value_guard).replace(value);
        if replaced_value.is_some() {
            self.size.fetch_add(1, Ordering::Relaxed);
        }
        replaced_value
    }

    fn hash(&self, value: &T) -> usize
        where T: Hash
    {
        let mut state = self.hash_builder.build_hasher();
        value.hash(&mut state);
        state.finish() as usize
    }

    fn resize(&self) {

    }
}
