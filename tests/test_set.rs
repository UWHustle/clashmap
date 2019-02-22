#[cfg(test)]
mod set_tests {

    use clashmap::set::ConcurrentHashSet;

    #[test]
    fn capacity() {
        let set: ConcurrentHashSet<i32> = ConcurrentHashSet::with_capacity(100);
        assert!(set.capacity() >= 100);
    }

    #[test]
    fn reserve() {
        let set: ConcurrentHashSet<i32> = ConcurrentHashSet::new();
        set.reserve(10);
        assert!(set.capacity() >= 10);
    }

    #[test]
    fn len() {
        let v = ConcurrentHashSet::new();
        assert_eq!(v.len(), 0);
        v.insert(1);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn is_empty() {
        let v = ConcurrentHashSet::new();
        assert!(v.is_empty());
        v.insert(1);
        assert!(!v.is_empty());
    }

    #[test]
    fn insert() {
        let set = ConcurrentHashSet::new();
        assert_eq!(set.insert(2), true);
        assert_eq!(set.insert(2), false);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn replace() {
        let set = ConcurrentHashSet::new();
        set.insert(Vec::<i32>::new());
        assert_eq!(set.get(&vec![]).unwrap().capacity(), 0);
        set.replace(Vec::with_capacity(10));
        assert_eq!(set.get(&vec![]).unwrap().capacity(), 10);
    }

    #[test]
    fn clear() {
        let v = ConcurrentHashSet::new();
        v.insert(1);
        v.clear();
        assert!(v.is_empty());
    }
}
