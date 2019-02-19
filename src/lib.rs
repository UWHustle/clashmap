use std::collections::HashSet;
use std::ptr;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::mem;

struct Node<K, V> {
    key: K,
    value: V,
    next: *mut Node<K, V>,
    prev: *mut Node<K, V>
}

pub struct LinkedHashMap<K, V> {
    hash_set: HashSet<Box<Node<K, V>>>,
    head: *mut Node<K, V>,
}

impl<K: Hash, V> Hash for Node<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

impl<K: PartialEq, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: Eq, V> Eq for Node<K, V> {}

#[derive(Hash, PartialEq, Eq)]
struct Qey<Q: ?Sized>(Q);

impl<Q: ?Sized> Qey<Q> {
    fn from_ref(q: &Q) -> &Self { unsafe { mem::transmute(q) } }
}

impl<K, V, Q: ?Sized> Borrow<Qey<Q>> for Box<Node<K, V>> where K: Borrow<Q> {
    fn borrow(&self) -> &Qey<Q> {
        Qey::from_ref(self.key.borrow() )
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

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.hash_set.get(Qey::from_ref(k)).map(|node| &node.value)
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
        self.hash_set.insert( node );
    }
}
