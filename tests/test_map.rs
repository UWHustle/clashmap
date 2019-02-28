//#[cfg(test)]
//mod map_tests {
//
//    use clashmap::map::ConcurrentHashMap;
//
//    #[test]
//    fn capacity() {
//        let map: ConcurrentHashMap<i32, i32> = ConcurrentHashMap::with_capacity(100);
//        assert!(map.capacity() >= 100);
//    }
//
//    #[test]
//    fn reserve() {
//        let map: ConcurrentHashMap<i32, i32> = ConcurrentHashMap::new();
//        map.reserve(10);
//        assert!(map.capacity() >= 10);
//    }
//
//    #[test]
//    fn len() {
//        let v = ConcurrentHashMap::new();
//        assert_eq!(v.len(), 0);
//        v.insert(1, "a");
//        assert_eq!(v.len(), 1);
//    }
//
//    #[test]
//    fn is_empty() {
//        let v = ConcurrentHashMap::new();
//        assert!(v.is_empty());
//        v.insert(1, "a");
//        assert!(!v.is_empty());
//    }
//
//    #[test]
//    fn clear() {
//        let v = ConcurrentHashMap::new();
//        v.insert(1, "a");
//        v.clear();
//        assert!(v.is_empty());
//    }
//
//    #[test]
//    fn get() {
//        let map = ConcurrentHashMap::new();
//        map.insert(1, "a");
//        assert_eq!(map.get(&1).unwrap(), &"a");
//        assert!(map.get(&2).is_none());
//    }
//
//    #[test]
//    fn insert() {
//        let map = ConcurrentHashMap::new();
//        assert_eq!(map.insert(37, "a"), None);
//        assert_eq!(map.is_empty(), false);
//
//        map.insert(37, "b");
//        assert_eq!(map.insert(37, "c"), Some("b"));
//        assert_eq!(map.get(&37).unwrap(), "c");
//    }
//
//    #[test]
//    fn remove() {
//        let map = ConcurrentHashMap::new();
//        map.insert(1, "a");
//        assert_eq!(map.remove(&1), Some("a"));
//        assert_eq!(map.remove(&1), None);
//    }
//}
