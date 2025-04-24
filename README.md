# REDRUM Hash Analysis Tool

[![CI](https://github.com/tapiocaboy/redrum/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/tapiocaboy/redrum/actions)
[![codecov](https://codecov.io/gh/tapiocaboy/redrum/graph/badge.svg)](https://codecov.io/gh/tapiocaboy/redrum)

A command-line tool for analyzing SHA-256 hashes of words, with a focus on character-level analysis, frequency distribution, and hash pattern analysis.

## Features

- SHA-256 hashing of complete words
- Individual character hash analysis
- Character frequency analysis of hash outputs
- Combined hash analysis (word vs. concatenated character hashes)
- Character shifting analysis between different hash methods
- Command-line interface with customizable input

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs)
2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd redrum
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

The tool can be used in several ways:

### Default Usage (REDRUM)
```bash
cargo run
```

### Custom Word
```bash
cargo run -- --word HELLO
# or
cargo run -- -w HELLO
```

### Help
```bash
cargo run -- --help
```

### Version
```bash
cargo run -- --version
```

## Output Format

The tool provides five types of analysis:

1. **Full SHA-256 Hash**: The complete hash of the input word
2. **Individual Character Hashes**: SHA-256 hash for each character in the word
3. **Frequency Analysis**: Character frequency distribution in the full hash
4. **Combined Hash Analysis**: Comparison between word hash and concatenated character hashes
5. **Character Shifting Analysis**: Shows how characters shift positions in different hash methods

## Example Output

```
# Hash Analysis of "REDRUM"

## 1. Full SHA-256 Hash
```
701b3c11c5c6d74b60f5ec1c0a8c298954ae7848a1a5951fecbda66a2a258338
```

## 2. Individual Character Hashes
| Character | SHA-256 Hash |
|-----------|--------------|
| R | 8c2574892063f995fdf756bce07f46c1a5193e54cd52837ed91e32008ccf41ac |
| E | a9f51566bd6705f7ea6ad54bb9deb449f795582d6529a0e22207b8981233ec58 |
...

## 3. Frequency Analysis of Full Hash
| Character | Frequency |
|-----------|-----------|
| 0 | 3 |
| 1 | 6 |
...

## 4. Combined Hash Analysis
Word Hash: `701b3c11c5c6d74b60f5ec1c0a8c298954ae7848a1a5951fecbda66a2a258338`
Combined Hash: `b015314e9e4693d686f33c08812b78b56563b8679a0074444fb9ac20bbe51f4e`

## 5. Character Shifting Analysis
| Character | Alphabet Index | Word Hash Char | Combined Hash Char |
|-----------|----------------|----------------|-------------------|
| R | 17 | 7 | b |
| E | 4 | 0 | 0 |
...
```

## Analysis Details

### Combined Hash Analysis
This analysis compares two different hashing approaches:
1. Hashing the entire word at once
2. Hashing each character individually, concatenating the results, and then hashing the concatenation

This comparison reveals patterns in how SHA-256 treats whole words versus their individual components.

### Character Shifting Analysis
Shows how each character's position shifts in different contexts:
- Original character's position in the alphabet (0-25)
- Corresponding character in the word hash
- Corresponding character in the combined hash

This analysis helps visualize the diffusion patterns in SHA-256 hashing.

## Dependencies

- `sha2`: For SHA-256 hashing
- `clap`: For command-line argument parsing

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Testing

The project includes both unit tests and doctests:

### Running Tests
```bash
# Run all tests (including doctests)
cargo test

# Run only unit tests
cargo test --lib

# Run only doctests
cargo test --doc
```

### Test Coverage
The project uses Codecov to track test coverage. The coverage badge at the top of this README shows the current coverage percentage.

To generate a local coverage report:
```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin
``` 
