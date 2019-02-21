use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::sync::{Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::MutexGuard;
use std::sync::RwLockWriteGuard;
use std::f32::MIN;
use std::mem;

const MIN_CAPACITY: usize = 2^4;
const MAX_LOAD_NUM: usize = 1;
const MAX_LOAD_DEN: usize = 2;

struct Bucket<T> {
    value: Option<T>
}

struct Table<T> {
    hash_builder: RandomState,
    buckets: Vec<RwLock<Bucket<T>>>
}

pub struct ConcurrentHashSet<T> {
    table: RwLock<Table<T>>,
    size: AtomicUsize,
}

impl<T> Bucket<T> {
    fn new() -> Self {
        Bucket {
            value: None
        }
    }
}

impl<T> Table<T> {
    fn with_capacity(capacity: usize) -> Self {
        Table {
            hash_builder: RandomState::new(),
            buckets: (0..capacity).map(|_| RwLock::new(Bucket::new())).collect()
        }
    }

    fn find_mut(&self, value: &T) -> RwLockWriteGuard<Bucket<T>>
        where T: Hash + Eq
    {
        let hash = self.hash(value);
        let mut i = hash % self.buckets.len();
        loop {
            let bucket_guard = self.buckets[i].write().unwrap();
            if bucket_guard.value.is_none() || bucket_guard.value.as_ref().unwrap() == value {
                break bucket_guard;
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
            table: RwLock::new(Table::with_capacity(MIN_CAPACITY)),
            size: AtomicUsize::new(0)
        }
    }

    pub fn capacity(&self) -> usize {
        self.table.read().unwrap().buckets.len() * MAX_LOAD_NUM / MAX_LOAD_DEN
    }

    pub fn reserve(&self, additional: usize)
        where T: Hash + Eq
    {
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
        let mut bucket_guard = table_guard.find_mut(&value);
        let replaced_value = bucket_guard.value.replace(value);
        if replaced_value.is_some() {
            self.size.fetch_add(1, Ordering::Relaxed);
        }
        replaced_value
    }

    fn resize(&self, new_raw_capacity: usize)
        where T: Hash + Eq
    {
        let mut table_guard = self.table.write().unwrap();
        if table_guard.buckets.len() < new_raw_capacity {
            let old_table = mem::replace(&mut *table_guard, Table::with_capacity(new_raw_capacity));
            for bucket in old_table.buckets {
                if let Some(value) = bucket.into_inner().unwrap().value {
                    let mut bucket_guard = table_guard.find_mut(&value);
                    bucket_guard.value.replace(value);
                }
            }
        }

//        if self.buckets.len() < new_raw_capacity {
//            let new_buckets = (0..new_raw_capacity).map(|_| Bucket::new()).collect();
//            for bucket in self.buckets {
//                if let Some(value) = *bucket.value.lock().unwrap() {
//
//                }
//            }
//        }
    }
}
