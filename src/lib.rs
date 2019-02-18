use std::collections::HashMap;
use std::ptr;
use std::hash::Hash;

struct Node<K, V> {
    key: K,
    value: V,
    next: *mut Node<K, V>,
    prev: *mut Node<K, V>
}

pub struct LinkedHashMap<K, V> {
    hash_map: HashMap<K, Node<K, V>>,
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>
}

impl<K: Hash + Eq, V> LinkedHashMap<K, V> {
    pub fn new() -> Self {
        LinkedHashMap {
            hash_map: HashMap::new(),
            head: ptr::null_mut(),
            tail: ptr::null_mut()
        }
    }
}
