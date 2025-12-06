# qrosity
A terminal and desktop application for generating QR codes with advanced customization options.

## Features
- Generate QR codes from text, URLs, and other data.
- Customize colors, sizes, and error correction levels.
- Optional GUI, CLI and batch processing support.
- Output formats: PNG, SVG (with svg feature enabled).

## Installation
Using `cargo`:
```bash
cargo install --git https://github.com/amaterasu-uwu-xd/qrosity
```

From source:
```bash
git clone https://github.com/amaterasu-uwu-xd/qrosity
cd qrosity
cargo build --release
```

> [!NOTE]
> By default, only the CLI feature is enabled, with SVG support, and without GUI or batch processing.
> To enable additional features, use the `--features` flag

## Usage
### Command Line Interface (CLI)
To generate a QR code from the command line, use the following command:
```bash
qrosity "Text or URL to encode" --output output.png
# You can also send data via stdin
echo "Text or URL to encode" | qrosity -o output.png
```

Use the `--help` flag to see all available options.

### Batch Processing
To use the batch processing feature, you need to have the batch feature enabled during installation with `--features batch`.
You can generate multiple QR codes in a batch by providing a JSON file with the required data:
```json
[
  {
    "Text": {
      "text": "https://example.com"
    },
    "output": "example_url.png",
    "ppm": 20,
    "foreground": ["#FF0000", "#0000FF"],
    "gradient_direction": "LeftToRight"
  },
  {
    "Wifi": {
      "ssid": "MyHomeNetwork",
      "security": "WPA",
      "password": "securepassword",
      "hidden": false
    },
    "output": "wifi_config.svg",
    "shape": "Dots",
    "finder": "Rounded"
  },
  {
    "Email": {
      "to": "contact@example.com",
      "subject": "Hello",
      "body": "This is a test email."
    },
    "output": "email_qr.png",
    "background": "#FFFF00"
  }
]
```
You can then run the batch processing command:
```bash
qrosity batch data.json
```

> [!CAUTION]
> The batch processing feature is still in development, major changes may occur.

### Graphical User Interface (GUI)
To launch the GUI application is necessary to have the GUI feature enabled during installation with `--features gui`.
```bash
qrosity gui
```

> [!CAUTION]
> The GUI feature is still in development, it just don't work.
