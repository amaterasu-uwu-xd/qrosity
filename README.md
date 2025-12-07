# qrosity
A terminal and desktop application for generating QR codes with advanced customization options.
![example qr code](./assets/example.png)

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
      "text": "Hello, World!",
      "ppm": 20,
      "foreground": "#000000",
      "output": "hello_world_qr.svg"
    }
  },
  {
    "Wifi": {
      "ssid": "MyNetwork",
      "password": "SecurePass1234",
      "security": "WPA",
      "hidden": false,
      "ppm": 25,
      "foreground": [
        "#00FF00",
        "#FFFF00"
      ],
      "output": "wifi_qr_alter.svg",
      "gradient_direction": "TopToBottom"
    }
  }
]
```
You can then run the batch processing command:
```bash
qrosity batch data.json
```

All the options available in the CLI are also available in batch processing.

> [!CAUTION]
> The batch processing feature is still in development, major changes may occur.

### Graphical User Interface (GUI)
To launch the GUI application is necessary to have the GUI feature enabled during installation with `--features gui`.
```bash
qrosity gui
```

> [!CAUTION]
> The GUI feature is still in development, it just don't work.

## Usage as a Library
You can also use `qrosity` as a library in your Rust projects. Add the following to your `Cargo.toml`:
```toml
[dependencies]
qrosity = { version = "*", no-default-features = true, features = ["svg"] } # Use no-default-features to avoid pulling in CLI/GUI dependencies. You can enable features as needed.
```

Then, you can use it in your code:
```rust
use qrosity::{core::to_qr, models::{EmailQr, QrConfig}};

fn main() {
    println!("Hello, world!");
    let email_qr = EmailQr {
        to: "example@example.com".to_string(),
        subject: Some("Greetings".to_string()),
        body: Some("Hello, this is a test email.".to_string()),
        cc: None,
        bcc: None,
        config: QrConfig {
            output: "help.png".to_string(),
            ecl: qrosity::core::QrCodeEcc::Medium,
            ..Default::default()
        }
    }; 
    println!("Generated mailto URI: {}", email_qr);
    to_qr(email_qr);
}
```

> [!CAUTION]
> This may change in future releases.