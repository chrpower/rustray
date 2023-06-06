pub fn random_usize() -> usize {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    let time = SystemTime::now();
    let mut hasher = DefaultHasher::new();

    time.hash(&mut hasher);

    hasher.finish() as usize
}

#[cfg(test)]
mod tests {
    use crate::util::random_usize;

    #[test]
    fn test_random_usize_in_range() {
        for _ in 0..100 {
            let result = random_usize();
            assert!(result <= usize::MAX);
        }
    }

    #[test]
    fn test_random_usize_not_same() {
        let result1 = random_usize();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let result2 = random_usize();
        assert_ne!(result1, result2);
    }
}
