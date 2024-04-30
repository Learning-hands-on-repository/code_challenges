use std::io::{self, Read};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (flag, contents, filename) = get_contents(args);

    let (line_count, word_count, byte_count) = process_content(&contents);

    match flag.as_str() {
        "-c" => println!("Character count: {}", byte_count),
        "-l" => println!("Line count: {}", line_count),
        "-w" => println!("Word count: {}", word_count),
        "" => println!("{} {} {} {}", line_count, word_count, byte_count, filename),
        _ => eprintln!("Invalid flag"),
    }
}

fn get_contents(args: Vec<String>) -> (String, String, String) {
    // TODO: Unit test stdin
    let mut contents = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut contents).unwrap();

    match contents.is_empty() {
        true => {
            let (flag, filename) = get_flag_and_filename_from(args);
            let (contents, filename) = get_contents_from(filename);
            (flag, contents, filename)
        }
        false => {
            let flag = get_flag(args);
            (flag, contents, "".to_string())
        }
    }
}

fn get_contents_from(filename: String) -> (String, String) {
    let contents = std::fs::read_to_string(filename.clone()).expect("Failed to read file");

    if contents.is_empty() {
        eprintln!("File is empty");
        std::process::exit(1);
    }

    (contents, filename)
}

pub fn process_content(contents: &str) -> (usize, usize, usize) {
    let line_count = contents.lines().count();
    let word_count = contents.split_whitespace().count();
    let byte_count = contents.len();
    (line_count, word_count, byte_count)
}

fn get_flag(args: Vec<String>) -> String {
    args[1].clone()
}

fn get_flag_and_filename_from(args: Vec<String>) -> (String, String) {
    if args.len() > 3 || args.len() < 2 {
        eprintln!("error: {}", "Usage: ccwc -[clw] <filename>");
        std::process::exit(1);
    }

    let flag;
    let filename;

    match args.len() {
        3 => {
            flag = args[1].clone();
            filename = args[2].clone();
        }
        _ => {
            flag = "".to_string();
            filename = args[1].clone();
        }
    }

    (flag, filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod process_content {
        use super::*;

        #[test]
        fn test_empty_content() {
            // Arrange
            let contents = "";

            // Act
            let (line_count, word_count, byte_count) = process_content(contents);

            // Assert
            assert_eq!(line_count, 0);
            assert_eq!(word_count, 0);
            assert_eq!(byte_count, 0);
        }

        #[test]
        fn test_single_line_content() {
            // Arrange
            let contents = "Hello, World!";

            // Act
            let (line_count, word_count, byte_count) = process_content(contents);

            // Assert
            assert_eq!(line_count, 1);
            assert_eq!(word_count, 2);
            assert_eq!(byte_count, 13);
        }

        #[test]
        fn test_multi_line_content() {
            // Arrange
            let contents = "Hello, World!\nHello, World!";

            // Act
            let (line_count, word_count, byte_count) = process_content(contents);

            // Assert
            assert_eq!(line_count, 2);
            assert_eq!(word_count, 4);
            assert_eq!(byte_count, 27);
        }
    }

    mod get_flag_filename_from {
        use super::*;

        #[test]
        fn test_valid_argument_count() {
            // Arrange
            let args = vec!["ccwc".to_string(), "filename.txt".to_string()];

            // Act
            let (flag, filename) = get_flag_and_filename_from(args);

            // Assert
            assert_eq!(flag, "");
            assert_eq!(filename, "filename.txt");
        }

        #[test]
        fn test_get_flag_filename_with_parameter() {
            // Arrange
            let args = vec![
                "ccwc".to_string(),
                "-c".to_string(),
                "filename.txt".to_string(),
            ];

            // Act
            let (flag, filename) = get_flag_and_filename_from(args);

            // Assert
            assert_eq!(flag, "-c");
            assert_eq!(filename, "filename.txt");
        }

        #[test]
        fn test_get_flag() {
            // Arrange
            let args = vec!["ccwc".to_string(), "-c".to_string()];

            // Act
            let flag = get_flag(args);

            // Assert
            assert_eq!(flag, "-c");
        }
    }
}
