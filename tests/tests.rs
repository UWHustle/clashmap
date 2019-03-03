extern crate omap;

#[cfg(test)]
mod tests {

    use omap::OrderedHashMap;

    #[test]
    fn iter() {
        let mut map = OrderedHashMap::new();
        map.insert_back("a", 1);
        map.insert_back("b", 2);
        map.insert_back("c", 3);

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&"a", &1)));
        assert_eq!(iter.next(), Some((&"b", &2)));
        assert_eq!(iter.next(), Some((&"c", &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn len() {
        let mut a = OrderedHashMap::new();
        assert_eq!(a.len(), 0);
        a.insert_back(1, "a");
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut a = OrderedHashMap::new();
        assert!(a.is_empty());
        a.insert_back(1, "a");
        assert!(!a.is_empty());
    }

    #[test]
    fn clear() {
        let mut a = OrderedHashMap::new();
        a.insert_back(1, "a");
        a.clear();
        assert!(a.is_empty());
    }

    #[test]
    fn get() {
        let mut map = OrderedHashMap::new();
        map.insert_back(1, "a");
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn next() {
        let mut map = OrderedHashMap::new();
        map.insert_back(1, "a");
        map.insert_back(2, "b");
        assert_eq!(map.next(&1), Some((&2, &"b")));
        assert_eq!(map.next(&2), None);
    }

    #[test]
    fn contains_key() {
        let mut map = OrderedHashMap::new();
        map.insert_back(1, "a");
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }

    #[test]
    fn insert_back() {
        let mut map = OrderedHashMap::new();
        assert_eq!(map.insert_back(37, "a"), None);
        assert_eq!(map.is_empty(), false);

        map.insert_back(37, "b");
        assert_eq!(map.insert_back(37, "c"), Some("b"));
        assert_eq!(map[&37], "c");
    }

    #[test]
    fn insert_front() {
        let mut map = OrderedHashMap::new();
        assert_eq!(map.insert_front(37, "a"), None);
        assert_eq!(map.is_empty(), false);

        map.insert_front(37, "b");
        assert_eq!(map.insert_front(37, "c"), Some("b"));
        assert_eq!(map[&37], "c");
    }

    #[test]
    fn insert_order() {
        let mut map = OrderedHashMap::new();
        map.insert_front("a", 1);
        map.insert_back("b", 2);
        map.insert_front("c", 3);
        map.insert_back("d", 4);

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&"c", &3)));
        assert_eq!(iter.next(), Some((&"a", &1)));
        assert_eq!(iter.next(), Some((&"b", &2)));
        assert_eq!(iter.next(), Some((&"d", &4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove() {
        let mut map = OrderedHashMap::new();
        map.insert_back(1, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&1), None);
    }
}
