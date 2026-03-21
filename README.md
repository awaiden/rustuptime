# rustuptime

A lightweight, high-performance website health checker built with Rust. `rustuptime` reads a list of URLs from a file and checks their status asynchronously, providing real-time feedback and response times.

## Features

- **Asynchronous Execution:** Uses `tokio` to check multiple sites concurrently for maximum efficiency.
- **Detailed Feedback:** Reports status codes, response times (ms), and connection errors.
- **Colorized Output:** Visual indicators for UP, DOWN, and ERROR states using `colored`.
- **Summary:** Provides a final report of total, successful, and failed checks.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
git clone https://github.com/awaiden/rustuptime.git
cd rustuptime
cargo build --release
```

## Usage

1. A sample `sites.txt` file is provided in the root directory. You can use it as a template or add your own URLs, one per line:

   ```text
   https://google.com
   https://github.com
   https://rust-lang.org
   https://this-site-should-fail.example
   ```

2. Run the tool:

   ```bash
   cargo run
   ```

## Example Output

```text
--- WEB HEALTH CHECK ---
[UP] https://google.com - 200 OK | (150)ms
[UP] https://github.com - 200 OK | (210)ms
[UP] https://rust-lang.org - 200 OK | (180)ms
[ERROR] https://this-site-should-fail.example - CONNECTION ERROR
Total sites: 4
Up sites: 3
Down sites: 1
```

## Tech Stack

- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [colored](https://github.com/mackwic/colored) - Terminal coloring
- [serde](https://github.com/serde-rs/serde) - Serialization/Deserialization

## License

MIT
