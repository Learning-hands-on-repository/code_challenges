fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        // write hint for ccwc -[clw] filename.txt
        eprintln!("Usage: ccwc -[clw] <filename>");
        std::process::exit(1);
    }

    let flag = &args[1];
    let filename = &args[2];

    let contents = std::fs::read_to_string(filename).expect("Failed to read file");
    if contents.is_empty() {
        eprintln!("File is empty");
        std::process::exit(1);
    }

    let (line_count, word_count, char_count) = process_content(&contents);
    match flag.as_str() {
        "-c" => println!("Character count: {}", char_count),
        "-l" => println!("Line count: {}", line_count),
        "-w" => println!("Word count: {}", word_count),
        _ => eprintln!("Invalid flag"),
    }
}

pub fn process_content(contents: &str) -> (usize, usize, usize) {
    let line_count = contents.lines().count();
    let word_count = contents.split_whitespace().count();
    let char_count = contents.chars().count();
    (line_count, word_count, char_count)
}

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

    #[test]
    fn test_word_count() {
        // Arrange
        let contents = "Hello, World!";

        // Act
        let (_, word_count, _) = process_content(contents);

        // Assert
        assert_eq!(word_count, 2);
    }

    #[test]
    fn test_char_count() {
        // Arrange
        let contents = "Hello, World!";

        // Act
        let (_, _, char_count) = process_content(contents);

        // Assert
        assert_eq!(char_count, 13);
    }
}
