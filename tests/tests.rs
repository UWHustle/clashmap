extern crate omap;

#[cfg(test)]
mod tests {

    use omap::OrderedHashMap;

    #[test]
    fn iter() {
        let mut map = OrderedHashMap::new();
        map.push_back("a", 1);
        map.push_back("b", 2);
        map.push_back("c", 3);

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
        a.push_back(1, "a");
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut a = OrderedHashMap::new();
        assert!(a.is_empty());
        a.push_back(1, "a");
        assert!(!a.is_empty());
    }

    #[test]
    fn clear() {
        let mut a = OrderedHashMap::new();
        a.push_back(1, "a");
        a.clear();
        assert!(a.is_empty());
    }

    #[test]
    fn get() {
        let mut map = OrderedHashMap::new();
        map.push_back(1, "a");
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn next() {
        let mut map = OrderedHashMap::new();
        map.push_back(1, "a");
        map.push_back(2, "b");
        assert_eq!(map.next(&1), Some((&2, &"b")));
        assert_eq!(map.next(&2), None);
    }

    #[test]
    fn first() {
        let mut map = OrderedHashMap::new();
        assert_eq!(map.first(), None);
        map.push_back(1, "a");
        map.push_back(2, "b");
        assert_eq!(map.first(), Some((&1, &"a")));
        map.remove(&1);
        assert_eq!(map.first(), Some((&2, &"b")));
    }

    #[test]
    fn last() {
        let mut map = OrderedHashMap::new();
        assert_eq!(map.last(), None);
        map.push_front(1, "a");
        map.push_front(2, "b");
        assert_eq!(map.first(), Some((&2, &"b")));
        map.remove(&2);
        assert_eq!(map.first(), Some((&1, &"a")));
    }

    #[test]
    fn contains_key() {
        let mut map = OrderedHashMap::new();
        map.push_back(1, "a");
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }

    #[test]
    fn insert_front() {
        let mut map = OrderedHashMap::new();
        assert_eq!(map.push_front(37, "a"), None);
        assert_eq!(map.is_empty(), false);

        map.push_front(37, "b");
        assert_eq!(map.push_front(37, "c"), Some("b"));
        assert_eq!(map[&37], "c");
    }

    #[test]
    fn insert_back() {
        let mut map = OrderedHashMap::new();
        assert_eq!(map.push_back(37, "a"), None);
        assert_eq!(map.is_empty(), false);

        map.push_back(37, "b");
        assert_eq!(map.push_back(37, "c"), Some("b"));
        assert_eq!(map[&37], "c");
    }

    #[test]
    fn insert_order() {
        let mut map = OrderedHashMap::new();
        map.push_front("a", 1);
        map.push_back("b", 2);
        map.push_front("c", 3);
        map.push_back("d", 4);

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
        map.push_back(1, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&1), None);
    }

    #[test]
    fn pop_front() {
        let mut map = OrderedHashMap::new();
        map.push_front(1, "a");
        map.push_front(2, "b");
        assert_eq!(map.pop_front(), Some((2, "b")));
        assert_eq!(map.pop_front(), Some((1, "a")));
        assert_eq!(map.pop_front(), None);
    }

    #[test]
    fn pop_back() {
        let mut map = OrderedHashMap::new();
        map.push_back(1, "a");
        map.push_back(2, "b");
        assert_eq!(map.pop_back(), Some((2, "b")));
        assert_eq!(map.pop_back(), Some((1, "a")));
        assert_eq!(map.pop_back(), None);
    }
}
