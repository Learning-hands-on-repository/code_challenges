#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_count() {
        // Arrange
        let contents = "Hello, World!\nHello, World!";

        // Act
        let (line_count, _, _) = process_content(contents);

        // Assert
        assert_eq!(line_count, 2);
    }
}
