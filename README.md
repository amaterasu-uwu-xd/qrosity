# qrosity
A versatile QR code generator library and application with many customization options.

Builded on top of the QR Code generator library by Nayuki (https://www.nayuki.io/page/qr-code-generator-library).

![example qr code](./assets/example.png)

## Features
- Generate QR codes from text, URLs, and other data.
- Customize colors, sizes, and error correction levels.
- Optional GUI, CLI and batch processing support.
- Output formats: SVG, EPS, PDF and some raster formats (PNG, JPEG, BMP, etc).

## Installation
Using `cargo`:
```bash
cargo install qrosity
```

From source:
```bash
git clone https://github.com/amaterasu-uwu-xd/qrosity
cd qrosity
cargo build --release
```

> [!NOTE]
> By default, only the CLI feature is enabled, without GUI or batch processing.
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
    "text": {
      "text": "Hello, World!",
      "foreground": ["#FF00FF", "#00FFFF"],
      "background": "#000000",
      "gradient-direction": "top-to-bottom"
      "shape": "horizontal-bars",
      "format": "svg"
    },
    "output": "greeting"
  },
  {
    "wifi": {
      "ssid": "MyNetwork",
      "password": "SecurePassword1234",
      "security": "wpa",
      "hidden": false
    },
    "output": "wifi-qr"
  },
  {
    "email": {
      "to": "example@mail.com",
      "subject": "Test Email",
      "body": "This is a test email generated from a QR code.",
      "icon": "example_icon.png",
      "format": "svg"
    },
    "output": "email-qr"
  }
]
```

If the `output` field does not contain an extension, the appropriate one will be added based on the specified format.

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

Then, you can use it in your code. The following example generates the example QR code shown at the top of this README:
```rust
use qrosity::{
    core::generate_qr,
    models::{FinderShape, GradientDirection, ModuleShape, OutputFormat, QrConfig, TextQr},
};

fn main() {
    let qr_struct = TextQr {
        text: Some("Hello, QR Code!".to_string()),
        config: QrConfig {
            background: "#1e1e2e".to_string(),
            foreground: vec![
                "#89b4fa".to_string(),
                "#74c7ec".to_string(),
                "#89dceb".to_string(),
                "#74c7ec".to_string(),
                "#89b4fa".to_string(),
            ],
            gradient_direction: GradientDirection::Radial,
            // Specify an icon path
            icon: Some("icon.png".to_string()),
            // Instead, if you use the `image` crate and have an image in memory, you can use:
            // image: QrImage::Raster(dynamic_image_variable), 
            shape: ModuleShape::Dots,
            finder: FinderShape::Rounded,
            format: OutputFormat::Svg,
            ..Default::default()
        },
    };

    let output_path = "output.svg";

    // Generate the QR code
    let result = generate_qr(&qr_struct).unwrap();

    // It can be saved to a file
    match result.save(output_path) {
        Ok(final_path) => println!("QR code saved to {}", final_path),
        Err(e) => eprintln!("Error saving QR: {}", e),
    }

    // Or get the raw bytes
    // Useful for web applications
    let qr_bytes = result.to_bytes().unwrap();
    println!("Generated QR code with {} bytes", qr_bytes.len());
}
```

> [!CAUTION]
> This may change in future releases.
