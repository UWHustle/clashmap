#[cfg(test)]
mod set_tests {

    use clashmap::set::ConcurrentHashSet;

    #[test]
    fn test_insert() {
        let mut set = ConcurrentHashSet::new();
        set.replace("value".to_string());
    }
}
