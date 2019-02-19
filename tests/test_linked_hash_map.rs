extern crate linked_hash_map;

#[cfg(test)]
mod tests {

    use linked_hash_map::LinkedHashMap;

    #[test]
    fn len() {
        let mut a = LinkedHashMap::new();
        assert_eq!(a.len(), 0);
        a.insert(1, "a");
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut a = LinkedHashMap::new();
        assert!(a.is_empty());
        a.insert(1, "a");
        assert!(!a.is_empty());
    }

    #[test]
    fn clear() {
        let mut a = LinkedHashMap::new();
        a.insert(1, "a");
        a.clear();
        assert!(a.is_empty());
    }

    #[test]
    fn get() {
        let mut map = LinkedHashMap::new();
        map.insert(1, "a");
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn insert() {
        let mut map = LinkedHashMap::new();
        assert_eq!(map.insert(37, "a"), None);
        assert_eq!(map.is_empty(), false);

        map.insert(37, "b");
        assert_eq!(map.insert(37, "c"), Some("b"));
        assert_eq!(map[&37], "c");
    }

    #[test]
    fn remove() {
        let mut map = LinkedHashMap::new();
        map.insert(1, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&1), None);
    }
}
