extern crate linked_hash_map;

#[cfg(test)]
mod tests {

    use linked_hash_map::LinkedHashMap;

    #[test]
    fn insert() {
        let mut map = LinkedHashMap::new();
        map.insert("key", b"value");
    }

    #[test]
    fn insert_get() {
        let mut map = LinkedHashMap::new();
        map.insert("key".to_string(), "a");
        assert_eq!(map.get("key"), Some(&"a"));
        assert_eq!(map.get("nonexistent_key"), None);
    }
}
