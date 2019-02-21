use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::sync::{RwLock, RwLockWriteGuard, atomic::{AtomicUsize, Ordering}};
use std::mem;
use std::borrow::Borrow;
use std::sync::RwLockReadGuard;

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

    fn find<Q: ?Sized>(&self, value: &Q) -> &RwLock<Bucket<T>>
        where T: Borrow<Q> + PartialEq<Q>,
              Q: Hash
    {
        let hash = self.hash(value);
        let mut i = hash % self.buckets.len();
        loop {
            let bucket = &self.buckets[i];
            let bucket_guard = bucket.read().unwrap();
            if bucket_guard.value.is_none() || bucket_guard.value.as_ref().unwrap() == value {
                break bucket;
            }
            i = (i + 1) % self.buckets.len();
        }
    }

    fn hash<Q: ?Sized>(&self, value: &Q) -> usize
        where T: Borrow<Q>,
              Q: Hash
    {
        let mut state = self.hash_builder.build_hasher();
        value.hash(&mut state);
        state.finish() as usize
    }
}

impl<T> ConcurrentHashSet<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ConcurrentHashSet {
            table: RwLock::new(Table::with_capacity(capacity * MAX_LOAD_DEN / MAX_LOAD_NUM)),
            size: AtomicUsize::new(0)
        }
    }

    pub fn capacity(&self) -> usize {
        self.table.read().unwrap().buckets.len() * MAX_LOAD_NUM / MAX_LOAD_DEN
    }

    pub fn reserve(&self, additional: usize)
        where T: Hash + PartialEq
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

//    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
//        where T: Borrow<Q> + PartialEq<Q>,
//              Q: Hash
//    {
//        self.table.read().unwrap().find(value).read().unwrap().
//    }

    pub fn insert(&self, value: T) -> bool
        where T: Hash + Eq
    {
        self.replace(value).is_none()
    }

    pub fn replace(&self, value: T) -> Option<T>
        where T: Hash + Eq
    {
        self.reserve(1);
        let table_guard = self.table.read().unwrap();
        let mut bucket_guard = table_guard.find(&value).write().unwrap();
        let replaced_value = bucket_guard.value.replace(value);
        if replaced_value.is_none() {
            self.size.fetch_add(1, Ordering::Relaxed);
        }
        replaced_value
    }

    fn resize(&self, new_raw_capacity: usize)
        where T: Hash + PartialEq
    {
        let mut table_guard = self.table.write().unwrap();
        if table_guard.buckets.len() < new_raw_capacity {
            let old_table = mem::replace(&mut *table_guard, Table::with_capacity(new_raw_capacity));
            for bucket in old_table.buckets {
                if let Some(value) = bucket.into_inner().unwrap().value {
                    let mut bucket_guard = table_guard.find(&value).write().unwrap();
                    bucket_guard.value.replace(value);
                }
            }
        }
    }
}
