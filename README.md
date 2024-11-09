# URL Shortener CLI Tool

A simple command-line tool written in Rust to shorten URLs and look them up later. This project demonstrates URL hashing, file-based storage, and command-line interface handling, making it an ideal small project for learning Rust.

## Features

- **Shorten URLs**: Generate a short URL for a given original URL. If the original URL already has a short URL, the tool will return the existing short URL instead of creating a duplicate.
- **Look Up URLs**: Retrieve the original URL from a given shortened URL.
- **Persistent Storage**: Stores URL mappings in a JSON file, so mappings are saved between runs.

## Installation

To install this tool, you'll need [Rust](https://www.rust-lang.org/tools/install) installed on your machine. Then, clone this repository and build the project:

```bash
git clone https://github.com/your-username/url_shortener.git
cd url_shortener
cargo build --release
```

The compiled binary will be in `target/release/url_shortener`.

## Usage

Run the tool using Cargo or the compiled binary. The tool has two main commands:

1. **Shorten a URL**:
   ```bash
   cargo run -- shorten <original_url>
   ```

   If this URL has already been shortened, it will display the existing shortened URL.

2. **Look up a URL**:
    Example:
   ```bash
   cargo run -- lookup <short_url>
   ```
  
## Example

```bash
$ cargo run -- shorten https://chatgpt.com/c/d7ea0561-8915-490c-9b6b-b38a5321a943
New Short URL is https://ac75da0f

$ cargo run -- shorten https://chatgpt.com/c/d7ea0561-8915-490c-9b6b-b38a5321a943
Short URL already exists: https://ac75da0f

$ cargo run -- lookup https://ac75da0f
Original URL: https://chatgpt.com/c/d7ea0561-8915-490c-9b6b-b38a5321a943
```

## Project Structure

- **`main.rs`**: Contains the main function and command-line interface logic.
- **`url_shortener.rs`**: Handles URL shortening and hashing logic.
- **`storage.rs`**: Manages loading and saving URL mappings in a JSON file.
- **`models.rs`**: Defines the data model for URL mappings.

## Configuration

The URLs are stored in a JSON file named `urls.json` in the project's root directory. You can change the file path by modifying the `DB_FILE` constant in `storage.rs`.

## Future Improvements

- **URL Expiration**: Add a feature to expire URLs after a certain period.
- **Custom Short URLs**: Allow users to specify custom short URLs.
- **Database Storage**: Support more scalable storage options (e.g., SQLite, PostgreSQL).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Feel free to submit issues or pull requests. Contributions are welcome!

