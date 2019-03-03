use std::collections::hash_map::RandomState;
use std::ptr;
use std::borrow::Borrow;
use std::hash::{BuildHasher, Hash, Hasher};
use std::mem;
use std::sync::{Arc, RwLock, RwLockReadGuard, Weak, atomic::{AtomicUsize, Ordering}};
use std::thread::current;

const MAX_LOAD_NUM: usize = 1;
const MAX_LOAD_DEN: usize = 2;

struct Record<K, V> {
    k: K,
    v: V
}

struct Node<K, V> {
    record: Option<Record<K, V>>,
    next: Option<Arc<RwLock<Node<K, V>>>>,
    prev: Weak<RwLock<Node<K, V>>>,
    edge: bool
}

struct Table<K, V> {
    hash_builder: RandomState,
    buckets: Vec<RwLock<Option<Arc<RwLock<Node<K, V>>>>>>
}

pub struct ConcurrentOrderedHashMap<K, V> {
    table: RwLock<Table<K, V>>,
    size: AtomicUsize,
    head: Option<Arc<RwLock<Node<K, V>>>>,
    tail: RwLock<Option<Weak<RwLock<Node<K, V>>>>>
}

impl<K, V> Record<K, V> {
    fn new(k: K, v: V) -> Self {
        Record { k, v }
    }
}

impl<K, V> Node<K, V> {
    fn new() -> Self {
        Node {
            record: None,
            next: None,
            prev: Weak::new(),
            edge: false
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

    fn len(&self) -> usize {
        self.buckets.len()
    }

    fn find<Q: ?Sized>(&self, k: &Q) -> Arc<RwLock<Node<K, V>>>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        let mut i = self.hash(k) % self.len();

        // Loop until an empty bucket or a bucket containing the key is found.
        loop {
            let bucket = &self.buckets[i];
            let mut bucket_guard = bucket.write().unwrap();

            if let Some(node) = &*bucket_guard {
                // The bucket is not empty. Check if the record's key equals the search key.
                let node_guard = node.read().unwrap();
                if let Some(record) = &node_guard.record {
                    if k.eq(record.k.borrow()) {
                        break node.clone()
                    }
                }
            } else {
                // The bucket is empty. Fill it with an empty node and return the node.
                let node = Arc::new(RwLock::new(Node::<K, V>::new()));
                bucket_guard.replace(node.clone());
                break node
            }
            i = (i + 1) % self.len();
        }
    }

    fn hash<Q: ?Sized>(&self, k: &Q) -> usize
        where K: Borrow<Q>,
              Q: Hash
    {
        let mut state = self.hash_builder.build_hasher();
        k.hash(&mut state);
        state.finish() as usize
    }
}

impl<K: Hash + Eq, V> ConcurrentOrderedHashMap<K, V> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ConcurrentOrderedHashMap {
            table: RwLock::new(Table::with_capacity(capacity * MAX_LOAD_DEN / MAX_LOAD_NUM)),
            size: AtomicUsize::new(0),
            head: None,
            tail: RwLock::new(None)
        }
    }

    pub fn capacity(&self) -> usize {
        self.table.read().unwrap().len() * MAX_LOAD_NUM / MAX_LOAD_DEN
    }

    pub fn reserve(&self, additional: usize) {
        if self.capacity() - self.len() < additional {
            let new_raw_capacity = (self.len() + additional) * MAX_LOAD_DEN / MAX_LOAD_NUM;
            self.resize(new_raw_capacity);
        }
    }

    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert_back(&self, k: K, v: V) -> Option<V> {
        self.reserve(1);

        let table_guard = self.table.read().unwrap();
        let node = table_guard.find(&k);
        let node_guard = node.write().unwrap();

        let next = if let Some(next_node) = &node_guard.next {
            let next_node_guard = next_node.write().unwrap();
            if next_node_guard.record.is_some() {
                Some(next_node.clone())
            } else if let Some(next_next_node) = &next_node_guard.next {
                Some(next_next_node.clone())
            } else {
                None
            }
        } else {
            None
        };

        let prev = if node_guard.edge {
            Some(node_guard.prev.upgrade().unwrap().clone())
        } else {
            None
        };

        // If the node if flagged as an edge, it is safe to request a lock from the previous node.
//        let prev_node = if node_guard.edge {
//            node_guard.prev.upgrade().unwrap().clone()
//        } else {
//            node.clone()
//        };
//
//        let mut next_node = &node_guard.next;



//        let next_node_guard = loop {
//            if let Some(current_node) = next_node {
//                let current_node_guard = &current_node.read().unwrap();
//                if current_node_guard.record.is_some() {
//                    break Some(current_node_guard)
//                }
//            } else {
//                break None
//            }
//
//        };

//        if let Some(current_node) = next_node {
//            let current_node_guard = current_node.read().unwrap();
//            if current_node_guard.record.is_none() {
//                next_node = &current_node_guard.next
//            }
//        }

//        while let Some(current_node) = next_node {
//            let current_node_guard = current_node.read().unwrap();
//            if current_node_guard.record.is_none() {
//                next_node = &current_node_guard.next;
//            } else {
//                break
//            }
//        }


        None
    }

    fn resize(&self, new_raw_capacity: usize) {
        let mut table_guard = self.table.write().unwrap();
        if table_guard.len() < new_raw_capacity {
            let old_table = mem::replace(&mut *table_guard, Table::with_capacity(new_raw_capacity));
            for mut bucket_lock in old_table.buckets {
                let mut bucket_guard = bucket_lock.write().unwrap();
                if let Some(node) = bucket_guard.take() {
                    let mut record_guard = node.write().unwrap();
                    if let Some(record) = record_guard.record.take() {
                        let mut node = table_guard.find(&record.k);
                        node.write().unwrap().record.replace(record);
                    }
                }
            }
        }
    }
}
