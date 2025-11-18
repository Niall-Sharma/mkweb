# Markdown-to-HTML Converter (Rust)

A simple command-line tool written in Rust that converts Markdown files into HTML files. It uses `pulldown-cmark` for parsing and supports building an entire directory of Markdown files into a corresponding set of HTML outputs.

## Features

- Recursive collection of Markdown files from an input directory
- Conversion from `.md` to `.html` using `pulldown-cmark`
- Automatic output directory creation
- Optional clean build
- Simple, type-safe CLI built with `clap`
- Extendable build pipeline suitable for static-site workflows

## Installation

Clone the repository and build the project:

```shell
git clone https://github.com/Niall-Sharma/mkweb.git
cd <project-directory>
cargo build --release
```

The compiled binary will appear in `target/release/`.

## Usage

### Basic build

Convert an entire Markdown directory into HTML:

```shell
./mkweb --input ./notes
```

### Specify an output directory

```shell
./mkweb --input ./notes --output ./public
```

### Clean output directory before building

```shell
./mkweb --input ./notes --output ./public --clean
```

## CLI Options

| Flag / Option       | Description                                         |
|---------------------|-----------------------------------------------------|
| `-i, --input`       | Path to the directory containing Markdown files     |
| `-o, --output`      | Output directory for generated HTML (default: `public`) |
| `--clean`           | Remove the output directory before generating files |
| `build`             | Run the build pipeline                              |

Example full command:

```shell
mkweb --input ./content --output ./site build
```

## Project Structure

```bash
src/
main.rs # CLI + command routing
builder.rs # Directory scanning and file operations
parser.rs # Markdown → HTML conversion logic
```

## How It Works

1. **Collect Markdown files:**  
   The program walks the input directory and finds all files ending in `.md`.

2. **Parse Markdown:**  
   Each file’s contents are fed into `pulldown-cmark::Parser`.

3. **Generate HTML:**  
   The converted HTML is written into a corresponding `.html` file in the output directory.

4. **Output result:**  
   If `--clean` is passed, the output directory is recreated fresh before writing.

## Requirements

- Rust 1.70+ (recommended)
- macOS, Linux, or Windows

## Future Improvements

- [ ] Checkbox support
- [ ] Table support
- [ ] Front-matter parsing (YAML/JSON)
- [ ] Incremental builds
- [ ] File watcher for live-rebuild

## License

MIT License. See `LICENSE` for details.
