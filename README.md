# Emoji Smuggler 🕵️

Educational demonstration of Unicode variation selector-based data smuggling.

## What is This?

The Emoji Smuggler App demonstrates how Unicode variation selectors can be exploited to hide arbitrary data within what appears to be a single emoji character. This technique can bypass AI safety filters and create covert communication channels.

**⚠️ FOR EDUCATIONAL AND DEFENSIVE SECURITY PURPOSES ONLY**

## Project Structure

```
emoji-smuggler/
├── src/
│   ├── lib.rs           # Core encoding/decoding functions
│   └── main.rs          # CLI demonstration tool
├── tests/
│   └── demo.rs          # Presentation demonstration tests
├── www/                 # Browser-based interactive demo
│   ├── index.html
│   ├── app.js
│   └── styles.css
├── scripts/             # Demo execution scripts
│   ├── run-terminal-demo.sh
│   └── build-and-serve-browser-demo.sh
└── pkg/                 # WASM build output (generated)
```

## Getting Started: Setup and Verification

This guide will walk you through setting up the project and verifying that everything is working correctly.

### Prerequisites

Before you begin, you need to have the **Rust toolchain** installed. If you don't have it, you can install it from [rustup.rs](https://rustup.rs/). The setup script will handle the rest.

### 1. Automated Setup

A comprehensive setup script is provided to prepare the development environment. This is the recommended way to get started.

Run the script from the root of the project:
```bash
./scripts/setup.sh
```

This script will perform the following actions:
- **Update Rust Toolchain**: Ensures you have the latest stable version of Rust.
- **Install `wasm-pack`**: A necessary tool for building the WebAssembly module for the browser demo.
- **Install `uv`**: A fast, modern Python package manager.
- **Create Python Virtual Environment**: Sets up an isolated `.venv` directory for Python tools.

All Rust crate dependencies (like `serde`, `wasm-bindgen`, etc.) are managed automatically by `cargo`, Rust's build tool and package manager. They will be downloaded and compiled the first time you build the project or run tests.

### 2. Verification

After the setup script completes, follow these steps to ensure the project is fully functional:

**A. Activate the Python Virtual Environment**

This is needed to use the web server for the browser demo.
```bash
source .venv/bin/activate
```

**B. Run the Test Suite**

Run all the tests to confirm that the core Rust logic is working correctly.
```bash
cargo test --all-features
```
You should see all tests passing.

**C. Run the Browser Demo**

This is the ultimate test, as it verifies the entire toolchain, including the WebAssembly build process.
```bash
./scripts/build-and-serve-browser-demo.sh
```
This will build the project and start a local web server. Open your browser to `http://localhost:8000/www/` to see the interactive demo.

Once you have completed these steps, your environment is fully configured and verified!

## Quick Start

### Terminal Demonstrations

Run the complete presentation demo suite:

```bash
./scripts/run-terminal-demo.sh
```

Or run individual commands:

```bash
# Build the project
cargo build --release

# Run full demonstration
cargo run --release -- demo

# Encode a message
cargo run --release -- encode 😊 "hello"

# Decode a message
cargo run --release -- decode "<encoded-emoji>"

# Run presentation tests
cargo test --test demo -- --nocapture
```

### Browser Demonstration

Build and serve the interactive web demo:

```bash
./scripts/build-and-serve-browser-demo.sh
```

Then open your browser to: `http://localhost:8000/www/`

The browser demo features:
- **Emoji Picker**: Select from a grid of popular emojis to use as the carrier.
- **Detailed Analysis**: View the exact character codes and UTF-8 bytes for every hidden payload.
- **Live Attack Simulation**: See how security filters and AI models interpret the smuggled data differently.

## How It Works

### Encoding

1. Takes a base character (typically an emoji) and a message
2. Converts each byte of the message to a Unicode variation selector:
   - Bytes 0-15 → U+FE00 to U+FE0F (Variation Selectors 1-16)
   - Bytes 16-255 → U+E0100 to U+E01EF (Variation Selectors 17-256)
3. Concatenates variation selectors after the base character
4. Result looks like one character but contains hidden data

### Decoding

1. Scans input string for variation selector characters
2. Converts each variation selector back to its byte value
3. Reconstructs the original message

### Attack Vector

- **What security filters see:** A single, innocent emoji character
- **What AI models receive:** Multiple characters with hidden payload
- **Result:** Token explosion, filter bypass, payload delivery

## Example Usage

### Rust API

```rust
use emoji_smuggler::{encode, decode};

// Encode
let hidden = encode('🧁', b"hello");
println!("{}", hidden); // Looks like: 🧁
println!("{:?}", hidden); // Shows hidden selectors

// Decode
let revealed = decode(&hidden);
assert_eq!(revealed, b"hello");
```

### WebAssembly (JavaScript)

```javascript
import init, { encode, decode } from './pkg/emoji_smuggler.js';

await init();

// Encode
const bytes = new TextEncoder().encode("hello");
const hidden = encode('🧁', bytes);

// Decode
const recovered = decode(hidden);
const message = new TextDecoder().decode(new Uint8Array(recovered));
```

## Testing

```bash
# Run all tests
cargo test

# Run library tests only
cargo test --lib

# Run presentation demo tests
cargo test --test demo -- --nocapture

# Run specific demo
cargo test --test demo demo_1_basic_encoding -- --nocapture
```

## Security Implications

### Attack Scenarios

1. **AI Jailbreaking**: Bypass safety filters with hidden instructions
2. **Data Exfiltration**: Smuggle data through DLP systems in "innocent" messages
3. **Malware C2**: Create covert command & control channels
4. **Social Engineering**: Hide malicious payloads in social media posts

### Defense Strategies

1. **Unicode Normalization**: Detect anomalous character sequences
2. **Variation Selector Filtering**: Strip or flag variation selectors
3. **Multi-Layer Inspection**: Analyze both visual and actual character counts
4. **Anomaly Detection**: Flag suspicious Unicode patterns

## Technical Details

- **Language**: Rust 2024 Edition (latest stable)
- **Minimum Rust Version**: 1.85.0
- **Tested with**: Rust 1.91.1 (November 2025)
- **Dependencies**:
  - wasm-bindgen 0.2.105
  - serde 1.0.228
  - serde-wasm-bindgen 0.6.5
  - console_error_panic_hook 0.1.7
- **Build Target**: Both native (CLI) and wasm32-unknown-unknown (browser)
- **WASM Size**: ~17KB (optimized)

## License

Educational and security research purposes only.

## Author

Gregory Kennedy
Pluralsight Author Audition - Emoji Hacking

## References

- [Unicode Variation Selectors](https://unicode.org/charts/PDF/UFE00.pdf)
- [Variation Selectors Supplement](https://unicode.org/charts/PDF/UE0100.pdf)
- [WASM Bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
