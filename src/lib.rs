use std::collections::HashSet;
use std::ptr;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

struct NodeRef<K, V> {
    node: Box<Node<K, V>>
}

struct Node<K, V> {
    key: K,
    value: V,
    next: *mut Node<K, V>,
    prev: *mut Node<K, V>
}

pub struct LinkedHashMap<K, V> {
    hash_set: HashSet<NodeRef<K, V>>,
    head: *mut Node<K, V>,
}

impl<K: Hash, V> Hash for NodeRef<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.key.hash(state)
    }
}

impl<K: PartialEq, V> PartialEq for NodeRef<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.node.key.eq(&other.node.key)
    }
}

impl<K: Eq, V> Eq for NodeRef<K, V> {}

impl<K, V> Borrow<K> for NodeRef<K, V> {
    fn borrow(&self) -> &K {
        &self.node.key
    }
}

impl<K, V> Node<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            value: v,
            next: ptr::null_mut(),
            prev: ptr::null_mut()
        }
    }
}

impl<K: Hash + Eq, V> LinkedHashMap<K, V> {
    pub fn new() -> Self {
        LinkedHashMap {
            hash_set: HashSet::new(),
            head: ptr::null_mut()
        }
    }

    pub fn get(&self, k: &K) -> Option<&V>
//        where K: Borrow<Q>,
//              Q: Eq + Hash
    {
        self.hash_set.get(k).map(|node| &(*node).node.value)
    }

    pub fn insert(&mut self, k: K, v: V) {
        let mut node = Box::new(Node::new(k, v));
        let raw_node: *mut _ = &mut *node;
        if self.head.is_null() {
            node.next = raw_node;
            node.prev = raw_node;
            self.head = raw_node;
        } else {
            unsafe {
                node.next = self.head;
                node.prev = (*self.head).prev;
                (*self.head).prev = raw_node;
                (*(*node).prev).next = raw_node;
            };
        }
        self.hash_set.insert(NodeRef { node: node });
    }
}
