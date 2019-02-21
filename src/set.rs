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

struct Table<T> {
    hash_builder: RandomState,
    buckets: Vec<Bucket<T>>
}

pub struct ConcurrentHashSet<T> {
    table: RwLock<Table<T>>,
    size: AtomicUsize,
}

impl<T> Bucket<T> {
    pub fn new() -> Self {
        Bucket {
            value: Mutex::new(None)
        }
    }
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Table {
            hash_builder: RandomState::new(),
            buckets: (0..MIN_CAPACITY).map(|_| Bucket::new()).collect()
        }
    }

    pub fn find(&self, value: &T) -> MutexGuard<Option<T>>
        where T: Hash + Eq
    {
        let hash = self.hash(value);
        let mut i = hash % self.buckets.len();
        loop {
            let value_guard = self.buckets[i].value.lock().unwrap();
            if (*value_guard).is_none() || (*value_guard).as_ref().unwrap() == value {
                break value_guard;
            }
            i = (i + 1) % self.buckets.len();
        }
    }

    fn hash(&self, value: &T) -> usize
        where T: Hash
    {
        let mut state = self.hash_builder.build_hasher();
        value.hash(&mut state);
        state.finish() as usize
    }
}

impl<T> ConcurrentHashSet<T> {
    pub fn new() -> Self {
        ConcurrentHashSet {
            table: RwLock::new(Table::new()),
            size: AtomicUsize::new(0)
        }
    }

    pub fn capacity(&self) -> usize {
        self.table.read().unwrap().buckets.len() * MAX_LOAD_NUM / MAX_LOAD_DEN
    }

    pub fn reserve(&self, additional: usize) {
        if self.capacity() - self.len() < additional {
            let raw_capacity = (self.len() + additional) * MAX_LOAD_DEN / MAX_LOAD_NUM;
            self.resize(raw_capacity);
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
        self.reserve(1);
        let table_guard = self.table.read().unwrap();
        let mut value_guard = table_guard.find(&value);
        let replaced_value = (*value_guard).replace(value);
        if replaced_value.is_some() {
            self.size.fetch_add(1, Ordering::Relaxed);
        }
        replaced_value
    }

    fn resize(&self, new_raw_capacity: usize) {
//        let mut buckets_guard = self.buckets.write().unwrap();
//        if buckets_guard.len() < new_raw_capacity {
////            let new_buckets = (0..new_raw_capacity).map(|_| Bucket::new()).collect();
////            for bucket in *buckets_guard {
////                if let Some(value) = *bucket.value.lock().unwrap() {
////
////                }
////            }
//        }
    }
}
