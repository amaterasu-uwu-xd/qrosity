# qrosity
A terminal and desktop application for generating QR codes with advanced customization options.

## Features
- Generate QR codes from text, URLs, and other data.
- Customize colors, sizes, and error correction levels.
- Optional GUI, CLI and batch processing support.
- Output formats: PNG, SVG (with svg feature enabled).

## Usage
### Command Line Interface (CLI)
To generate a QR code from the command line, use the following command:
```bash
qrosity "Text or URL to encode" --output output.png --ppm 300
# You can also send data via stdin
echo "Text or URL to encode" | qrosity --output output.png --ppm 300 
```

Use the `--help` flag to see all available options.

### Graphical User Interface (GUI)
To launch the GUI application is necessary to have the GUI feature enabled during installation with `--features gui`.
```bash
qrosity gui
```

> [!CAUTION]
> The GUI feature is still in development, it just don't work.

### Batch Processing
You can generate multiple QR codes in a batch by providing a text file with each line representing the data to encode:
```bash
qrosity batch --input data.txt --output-dir ./qrcodes
```
> [!CAUTION]
> The batch processing feature is still in development, it just don't work.

## Installation
Using `cargo`:
```bash
cargo install --git https://github.com/amaterasu-uwu-xd/qrosity
# If you want svg support
cargo install --git https://github.com/amaterasu-uwu-xd/qrosity --features svg
```

From source:
```bash
git clone https://github.com/amaterasu-uwu-xd/qrosity
cd qrosity
cargo build --release
# If you want svg support
cargo build --release --features svg
```