# Archive Tool

A command line tool for validating and archiving records stored in the database (written in Rust)

## Usage

### Using Docker

### Build from Source

Clone the Repository

```bash
git clone --branch rs https://github.com/GymSquad/archive-tool.git
```

Change Your Current Directory

```bash
cd archive-tool
```

Build the Binary

```bash
cargo build --release && cp target/release/archive-tool .
```

Run the Program

```bash
./archive-tool <pywb collections path>
```
