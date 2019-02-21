#[cfg(test)]
mod set_tests {

    use clashmap::set::ConcurrentHashSet;

    #[test]
    fn test_insert() {
        let set = ConcurrentHashSet::new();
        set.replace("value".to_string());
    }
}
