# TSC_GPT

A Rust CLI tool that parses TypeScript compiler (tsc) errors and provides simplified explanations using GPT.

## Features

- Runs the TypeScript compiler on your project
- Parses and formats TypeScript error messages
- Provides clear, concise explanations of each error using GPT
- Shows relevant code context for each error

## Installation

Requires Rust and TypeScript to be installed.

```bash
# Clone the repository
git clone https://github.com/yourusername/tsc_gpt.git
cd tsc_gpt

# Build the project
cargo build --release

# The binary will be available at target/release/tsc_gpt
```

## Usage

1. Set your OpenAI API key:

```bash
export OPENAI_API_KEY=your_api_key_here
```

2. Run from your TypeScript project directory:

```bash
tsc_gpt
```

## Requirements

- Rust 2021 edition or later
- TypeScript (`tsc` available in PATH)
- OpenAI API key

## License

MIT