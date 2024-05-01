Certainly! Below is a basic README for manually testing the Rust program you provided:

---

# ccwc - Character, Word, and Line Counter

ccwc is a simple command-line utility written in Rust for counting characters, words, and lines in text files.

## Usage

The program accepts the following command-line arguments:

```
ccwc [OPTION] [FILENAME]
```

- `[OPTION]` can be one of the following:
  - `-c`: Count characters
  - `-w`: Count words
  - `-l`: Count lines
- `[FILENAME]` is the path to the text file you want to analyze.

If no option is provided, ccwc will display the counts for all three metrics along with the filename.

## Installation

To use ccwc, you need to have Rust installed on your system. You can install Rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can compile the program using Cargo, Rust's package manager and build system:

```
cargo build --release
```

This will generate the executable file in the `target/release` directory.

## Example

Suppose you have a text file named `example.txt` with the following content:

```
Hello, world!
This is a sample text file.
```

To count the number of characters in the file, run:

```
./target/release/ccwc -c example.txt
```

Output:

```
Character count: 41
```

To count the number of lines in the file, run:

```
./target/release/ccwc -l example.txt
```

Output:

```
Line count: 2
```

To count the number of words in the file, run:

```
./target/release/ccwc -w example.txt
```

Output:

```
Word count: 8
```

To display counts for all three metrics along with the filename, run:

```
./target/release/ccwc example.txt
```

Output:

```
2 8 41 example.txt
```

## Unit Testing

Unit tests are provided to ensure the correctness of the program's functionalities. You can run the tests using:

```
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Feel free to adjust and expand this README according to your project's specific needs and features!
