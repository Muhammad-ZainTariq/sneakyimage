# sneakyimage

A simple, retro-style command-line tool for hiding and revealing secret messages inside images using steganography, written in Rust.

## Features

- Encode (hide) any secret message inside a PNG image
- Decode (reveal) the hidden message from an image
- No dependencies on special folders—works anywhere
- Fast, portable, and open source

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/Muhammad-ZainTariq/sneakyimage.git
cd sneakyimage
cargo build --release
```

To install globally (so you can use `sneakyimage` from anywhere):

```bash
cargo install --path .
```

## Usage

### Encode a message

```bash
sneakyimage enc <input_image> "<your secret message>" <output_image>
```

**Example:**
```bash
sneakyimage enc /Users/muhammad-zain/Documents/logo2.png "Yo wys say" /Users/muhammad-zain/Documents/logoo.png
```

**Or run directly from the target directory:**
```bash
./target/release/sneakyimage enc /Users/muhammad-zain/Documents/logo2.png "Yo wys say" /Users/muhammad-zain/Documents/logoo.png
```

### Decode a message

```bash
sneakyimage dec <encoded_image>
```

**Example:**
```bash
sneakyimage dec /Users/muhammad-zain/Documents/logoo.png
```

**Or run directly from the target directory:**
```bash
./target/release/sneakyimage dec /Users/muhammad-zain/Documents/logoo.png
```

## Output

When you encode, the output image will be saved wherever you specify.  
When you decode, the secret message will be printed to your terminal.

## License

MIT (or specify your preferred license)

![Rust](https://img.shields.io/badge/Rust-🦀-orange)
![Open Source](https://img.shields.io/badge/Open%20Source-Yes-brightgreen) 
