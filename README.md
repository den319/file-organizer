# File Organizer

A lightweight Rust CLI tool that automatically organizes files into categorized folders based on their file extensions.

## Features

- **Automatic Categorization** – Sorts files into folders by type:
  - 🖼️ **Images** – `.png`, `.jpg`, `.jpeg`, `.webp`
  - 🎬 **Videos** – `.mp4`, `.mkv`
  - 📄 **Documents** – `.pdf`, `.docx`
  - 📦 **Others** – Any unrecognized extensions
- **Dry-Run Mode** – Preview changes before moving anything
- **Skip Directories** – Only files are processed; subdirectories are left untouched
- **Duplicate Protection** – Skips files that already exist at the destination

## Usage

```
cargo run -- <PATH> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `<PATH>` | Path to the directory containing files to organize |

### Options

| Option | Description |
|--------|-------------|
| `-d`, `--dry-run` | Simulate the organization without moving any files |

### Examples

Organize files in the current directory:

```bash
cargo run -- .
```

Preview what would happen without actually moving files:

```bash
cargo run -- . --dry-run
```

Organize files in a specific folder:

```bash
cargo run -- ~/Downloads
```

## How It Works

1. Reads all entries in the specified directory
2. Skips any subdirectories
3. Determines each file's category based on its extension
4. Creates a category folder under `./test_folder/` (e.g., `./test_folder/Images/`)
5. Moves the file into the corresponding category folder
6. If a file with the same name already exists at the destination, it is skipped

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/den319/file-organizer.git
cd file-organizer
cargo build --release
```

The binary will be available at `target/release/file-organizer`.

## Project Structure

```
src/
├── main.rs         # Entry point – reads directory and dispatches files
├── organizer.rs    # File moving/renaming logic
├── categories.rs   # Extension-to-category mapping
└── config.rs       # CLI argument definitions (clap)
```

## Dependencies

- [clap](https://crates.io/crates/clap) – CLI argument parsing (v4.6+)