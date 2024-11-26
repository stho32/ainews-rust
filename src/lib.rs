/// Library crate for ainews-rust
/// Contains utility functions for number processing

/// Converts a string to an i32 and returns a Result
pub fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.trim().parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number(" 123 "), Ok(123));
        assert!(parse_number("abc").is_err());
    }
}
