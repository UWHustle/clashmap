use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::sync::{RwLock, atomic::{AtomicUsize, Ordering}};
use std::mem;
use std::borrow::Borrow;
use std::sync::RwLockReadGuard;
use owning_ref::{OwningHandle, OwningRef};
use std::ops::Deref;
use std::fmt::{Debug, Error, Formatter};

const MAX_LOAD_NUM: usize = 1;
const MAX_LOAD_DEN: usize = 2;

pub struct ValueGuard<'a, K, V: 'a>(
    OwningRef<OwningHandle<RwLockReadGuard<'a, Table<K, V>>, RwLockReadGuard<'a, Option<Bucket<K, V>>>>, V>
);

pub struct Bucket<K, V> {
    key: K,
    value: V
}

struct Table<K, V> {
    hash_builder: RandomState,
    buckets: Vec<RwLock<Option<Bucket<K, V>>>>
}

pub struct ConcurrentHashMap<K, V> {
    table: RwLock<Table<K, V>>,
    size: AtomicUsize,
}

impl<'a, K, V: Debug> Debug for ValueGuard<'a, K, V> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<'a, K, V> Deref for ValueGuard<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &V {
        &self.0
    }
}

impl<'a, K, V> PartialEq<V> for ValueGuard<'a, K, V>
    where V: Eq
{
    fn eq(&self, other: &V) -> bool {
        other.eq(self)
    }
}

impl<K, V> Bucket<K, V> {
    fn new(key: K, value: V) -> Self {
        Bucket {
            key,
            value
        }
    }
}

impl<K, V> Table<K, V> {
    fn with_capacity(capacity: usize) -> Self {
        Table {
            hash_builder: RandomState::new(),
            buckets: (0..capacity).map(|_| RwLock::new(None)).collect()
        }
    }

    fn find<Q: ?Sized>(&self, key: &Q) -> (usize, &RwLock<Option<Bucket<K, V>>>)
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        let hash = self.hash(key);
        let mut i = hash % self.buckets.len();
        loop {
            let bucket = &self.buckets[i];
            let bucket_guard = self.buckets[i].read().unwrap();
            if bucket_guard.is_none() || key.eq(bucket_guard.as_ref().unwrap().key.borrow()) {
                break (i, bucket);
            }
            i = (i + 1) % self.buckets.len();
        }
    }

    fn take_shift<Q: ?Sized>(&self, key: &Q) -> Option<Bucket<K, V>>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        let (i, bucket_lock) = self.find(key);
        let mut bucket_guard = bucket_lock.write().unwrap();

        bucket_guard.take().map(|bucket| {
            let mut swap_index = i;
            while self.buckets[(swap_index + 1) % self.buckets.len()].read().unwrap().is_some() {
                swap_index += 1;
            }

            if swap_index != i {
                let mut swap_bucket_guard = self.buckets[swap_index % self.buckets.len()].write().unwrap();
                mem::swap(&mut bucket_guard, &mut swap_bucket_guard);
            }
            bucket
        })
    }

    fn hash<Q: ?Sized>(&self, key: &Q) -> usize
        where K: Borrow<Q>,
              Q: Hash
    {
        let mut state = self.hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish() as usize
    }
}

impl<K, V> ConcurrentHashMap<K, V>
    where K: Eq + Hash
{
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ConcurrentHashMap {
            table: RwLock::new(Table::with_capacity(capacity * MAX_LOAD_DEN / MAX_LOAD_NUM)),
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

    pub fn clear(&self) {
        let mut table_guard = self.table.write().unwrap();
        mem::replace(&mut *table_guard, Table::with_capacity(0));
        self.size.store(0, Ordering::Relaxed);
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<ValueGuard<K, V>>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        let bucket_handle = OwningHandle::new_with_fn(
            self.table.read().unwrap(),
            |t| unsafe {&*t}.find(key).1.read().unwrap());

        match &*bucket_handle {
            Some(_) => Some(ValueGuard(OwningRef::new(bucket_handle).map(|b| &b.as_ref().unwrap().value))),
            None => None
        }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V>
        where K: Hash + Eq
    {
        self.reserve(1);
        let table_guard = self.table.read().unwrap();
        let mut bucket_guard = table_guard.find(&key).1.write().unwrap();
        let replaced_bucket = bucket_guard.replace(Bucket::new(key, value));
        replaced_bucket
            .map(|b| b.value)
            .or_else(|| {
                self.size.fetch_add(1, Ordering::Relaxed);
                None
            })
    }

    pub fn remove<Q: ?Sized>(&self, value: &Q) -> Option<V>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.table.read().unwrap().take_shift(value).map(|b| b.value)
    }

    fn resize(&self, new_raw_capacity: usize)
        where K: Hash
    {
        let mut table_guard = self.table.write().unwrap();
        if table_guard.buckets.len() < new_raw_capacity {
            let old_table = mem::replace(&mut *table_guard, Table::with_capacity(new_raw_capacity));
            for bucket in old_table.buckets {
                if let Some(b) = bucket.into_inner().unwrap().take() {
                    let mut bucket_guard = table_guard.find(&b.key).1.write().unwrap();
                    bucket_guard.replace(b);
                }
            }
        }
    }
}
