use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::sync::{RwLock, atomic::{AtomicUsize, Ordering}};
use std::mem;
use std::borrow::Borrow;
use std::sync::RwLockReadGuard;
use owning_ref::OwningRef;
use owning_ref::OwningHandle;
use std::ops::Deref;

const MAX_LOAD_NUM: usize = 1;
const MAX_LOAD_DEN: usize = 2;

pub struct ValueGuard<'a, T: 'a> {
    value: OwningRef<OwningHandle<RwLockReadGuard<'a, Table<T>>, RwLockReadGuard<'a, Bucket<T>>>, T>
}

pub struct Bucket<T> {
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

impl<'a, T> Deref for ValueGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<'a, T: PartialEq> PartialEq for ValueGuard<'a, T> {
    fn eq(&self, other: &ValueGuard<'a, T>) -> bool {
        self == other
    }
}

impl<'a, T: Eq> Eq for ValueGuard<'a, T> {}

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
              Q: Hash + Eq
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

impl<T> ConcurrentHashSet<T>
    where T: Eq + Hash
{
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

    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<ValueGuard<T>>
        where T: Borrow<Q> + PartialEq<Q>,
              Q: Hash + Eq
    {
//        if let Some(value) = OwningRef::new(
//            OwningHandle::new_with_fn(self.table.read().unwrap(), |t| unsafe { &*t }.find(value)))
//            .map(|b| &b.value) {
//            ValueGuard { value }
//        } else {
//            None
//        }
        let bucket_handle = OwningHandle::new_with_fn(
            self.table.read().unwrap(),
            |t| unsafe {&*t}.find(value).read().unwrap());

        match (*bucket_handle).value {
            Some(_) => Some(ValueGuard { value: OwningRef::new(bucket_handle).map(|b| b.value.as_ref().unwrap()) }),
            None => None
        }

//        let value_handle = OwningHandle::new_with_fn(
//            bucket_handle,
//            |b| unsafe {&*b}.value.as_ref());

//        match *value_ref {
//            Some(value) => Some(ValueGuard { value: value_ref.map(|v| &v.unwrap()) }),
//            None => None
//        };

//
//        let value_handle = OwningHandle::new_with_fn(
//            self.table.read().unwrap(),
//          |t| RwLockReadGuardRef::new(unsafe {&*t}.find(value).read().unwrap())
//              .map(|b| &b.value));
//        value_handle.deref().as_ref()
    }

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
