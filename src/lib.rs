use std::collections::HashSet;
use std::ptr;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::mem;
use std::ops::Index;

struct Node<K, V> {
    key: K,
    value: V,
    next: *mut Node<K, V>,
    prev: *mut Node<K, V>
}

pub struct LinkedHashMap<K, V> {
    hash_set: HashSet<Box<Node<K, V>>>,
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>
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
struct Key<Q: ?Sized>(Q);

impl<Q: ?Sized> Key<Q> {
    fn from_ref(q: &Q) -> &Self { unsafe { mem::transmute(q) } }
}

impl<K, V, Q: ?Sized> Borrow<Key<Q>> for Box<Node<K, V>> where K: Borrow<Q> {
    fn borrow(&self) -> &Key<Q> {
        Key::from_ref(self.key.borrow() )
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
            head: ptr::null_mut(),
            tail: ptr::null_mut()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hash_set.is_empty()
    }

    pub fn clear(&mut self) {
        self.hash_set.clear();
        self.head = ptr::null_mut();
        self.tail = ptr::null_mut();
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.hash_set.get(Key::from_ref(k)).map(|node| &node.value)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let mut node = Box::new(Node::new(k, v));
        let raw_node: *mut _ = &mut *node;
        if self.tail.is_null() {
            self.head = raw_node;
            self.tail = raw_node;
        } else {
            unsafe {
                node.prev = self.tail;
                (*self.tail).next = raw_node;
            };
        }
        self.hash_set.replace(node).map(|node| node.value)
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.hash_set.take(Key::from_ref(k)).map(|node| {
            if node.prev.is_null() {
                self.head = node.next
            } else {
                unsafe {
                    (*node.prev).next = node.next
                }
            }

            if node.next.is_null() {
                self.tail = node.prev
            } else {
                unsafe {
                    (*node.next).prev = node.prev
                }
            }

            node.value
        })
    }
}

impl<'a, K, V, Q: ?Sized> Index<&'a Q> for LinkedHashMap<K, V>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}
