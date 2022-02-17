/// Validates whether the type given is true
#[must_use]
pub fn validate_skip<'a, T>(_: T) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::validate_skip;

    #[derive(Debug, Default)]
    struct TestInner {
        array: [u8; 11],
    }

    #[test]
    fn test_validate_skip_alway_true() {
        let test = TestInner::default();

        assert_eq!(validate_skip("23"), true);
        assert_eq!(validate_skip(24), true);
        assert_eq!(validate_skip(test.array), true);
    }
}
