pub fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix('\n'))
        .unwrap_or(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_trailing_newline() {
        assert_eq!(strip_trailing_newline("test\r\n"), "test");
        assert_eq!(strip_trailing_newline("test\n"), "test");
        assert_eq!(strip_trailing_newline("test"), "test");
    }

    #[test]
    fn test_strip_trailing_newline_empty() {
        assert_eq!(strip_trailing_newline(""), "");
    }

    #[test]
    fn test_strip_trailing_newline_empty_newline() {
        assert_eq!(strip_trailing_newline("\n"), "");
    }

    #[test]
    fn test_strip_trailing_newline_empty_newline_carriage_return() {
        assert_eq!(strip_trailing_newline("\r\n"), "");
    }
}
