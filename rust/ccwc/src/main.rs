use std::io::{self, Read};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (reading_mode, option, filename) = get_reading_configuration(args);

    let (contents, filename) = get_contents(reading_mode, filename);

    let (line_count, word_count, byte_count) = get_content_information(&contents);

    match option.as_str() {
        "-c" => println!("Character count: {}", byte_count),
        "-l" => println!("Line count: {}", line_count),
        "-w" => println!("Word count: {}", word_count),
        "" => println!("{} {} {} {}", line_count, word_count, byte_count, filename),
        _ => eprintln!("Invalid flag"),
    }
}

fn get_reading_configuration(args: Vec<String>) -> (String, String, String) {
    if args.len() > 3 || args.len() < 2 {
        eprintln!("error: {}", "Usage: ccwc -[clw] <filename>");
        std::process::exit(1);
    }

    match args.len() {
        3 => ("from_file".to_string(), args[1].clone(), args[2].clone()),
        2 => {
            let reading_mode;
            if args[1].starts_with("-") {
                reading_mode = "from_stdin".to_string();
                return (reading_mode, args[1].clone(), "".to_string());
            } else {
                reading_mode = "from_file".to_string();
                return (reading_mode, "default".to_string(), args[1].clone());
            }
        }
        _ => ("".to_string(), "default".to_string(), "".to_string()),
    }
}

// TODO: How to test this?
fn get_contents(reading_mode: String, filename: String) -> (String, String) {
    match reading_mode.as_str() {
        "from_file" => {
            let (contents, filename) = get_contents_from(filename);
            (contents, filename)
        }
        "from_stdin" => {
            // TODO: Unit test stdin
            let mut contents = String::new();
            let stdin = io::stdin();
            // TODO: Handle empty stdin
            stdin.lock().read_to_string(&mut contents).unwrap();

            (contents, "".to_string())
        }
        _ => {
            eprintln!("Invalid reading mode");
            std::process::exit(1);
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

pub fn get_content_information(contents: &str) -> (usize, usize, usize) {
    let line_count = contents.lines().count();
    let word_count = contents.split_whitespace().count();
    let byte_count = contents.len();
    (line_count, word_count, byte_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_reading_mode {
        use super::*;

        #[test]
        fn test_get_reading_mode_and_option() {
            // Arrange
            let args = vec![
                "ccwc".to_string(),
                "-c".to_string(),
                "filename.txt".to_string(),
            ];

            // Act
            let (reading_mode, option, filename) = get_reading_configuration(args);

            // Assert
            assert_eq!(reading_mode, "from_file");
            assert_eq!(option, "-c");
            assert_eq!(filename, "filename.txt");
        }

        #[test]
        fn test_reading_file_mode_with_default() {
            // Arrange
            let args = vec!["ccwc".to_string(), "filename.txt".to_string()];

            // Act
            let (reading_mode, option, filename) = get_reading_configuration(args);

            // Assert
            assert_eq!(reading_mode, "from_file");
            assert_eq!(option, "default");
            assert_eq!(filename, "filename.txt");
        }

        #[test]
        fn test_reading_stdin_mode() {
            // Arrange
            let args = vec!["ccwc".to_string(), "-c".to_string()];

            // Act
            let (reading_mode, option, filename) = get_reading_configuration(args);

            // Assert
            assert_eq!(reading_mode, "from_stdin");
            assert_eq!(option, "-c");
            assert_eq!(filename, "");
        }
    }

    mod get_content_information {
        use super::*;

        #[test]
        fn test_empty_content() {
            // Arrange
            let contents = "";

            // Act
            let (line_count, word_count, byte_count) = get_content_information(contents);

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
            let (line_count, word_count, byte_count) = get_content_information(contents);

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
            let (line_count, word_count, byte_count) = get_content_information(contents);

            // Assert
            assert_eq!(line_count, 2);
            assert_eq!(word_count, 4);
            assert_eq!(byte_count, 27);
        }
    }
}
