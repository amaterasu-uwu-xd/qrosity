use crate::core::qrgen;

#[cfg(feature = "cli")]
use clap::{Args, ValueEnum};

/// QR code module shapes. 
/// These shapes determine how each module (square) of the QR code is rendered.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub enum ModuleShape {
    Square,
    Dots,
    Gapped,
    Heart,
    Diamond,
    HorizontalBars,
    VerticalBars,
}

// QR code finder pattern shapes. 
/// These shapes determine how the position detection patterns ("eyes") of the QR code are rendered.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub enum FinderShape {
    Square,
    Circle,
    Rounded,
}

/// QR code gradient directions.
/// These directions determine how color gradients are applied to the QR code.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub enum GradientDirection {
    TopToBottom,
    LeftToRight,
    TopLeftToBottomRight,
    BottomLeftToTopRight,
    Radial,
}

/// Configuration options for generating and rendering a QR code.
/// This struct holds various settings that control the appearance
/// and behavior of the generated QR code.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(Args))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "batch", serde(default))]
pub struct QrConfig {
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            default_value = "4",
            help = "Quiet zone size (0-10 modules)",
            value_parser = clap::value_parser!(u32).range(0..=10),
        )
    )]
    pub quiet_zone: u32,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            help = "Maximum QR Code version (1-40)",
            value_parser = clap::value_parser!(u8).range(1..=40)
        )
    )]
    pub max_version: Option<u8>,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            default_value = "medium",
            help = "Error correction level",
        )
    )]
    pub ecl: qrgen::QrCodeEcc,

    #[cfg_attr(
        feature = "cli", 
        arg(
            long, 
            help = "Mask pattern to use (0-7). If not set, it will be chosen automatically.",
            value_parser = clap::value_parser!(u8).range(0..=7)
        )
    )]
    pub mask: Option<u8>,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            help = "Foreground color(s). If multiple colors are provided, a gradient is created.",
            default_value = "#000000",
            num_args = 1..,
        )
    )]
    pub foreground: Vec<String>,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            help = "Background color in hex format",
            default_value = "#FFFFFF",
        )
    )]
    pub background: String,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            help = "Gradient direction",
            value_enum,
            default_value_t = GradientDirection::TopLeftToBottomRight,
        )
    )]
    pub gradient_direction: GradientDirection,

    // Pixels per module
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            help = "Pixels per module",
            default_value = "25",
            value_parser = clap::value_parser!(u32).range(25..=100),
        )
    )]
    pub ppm: u32,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            help = "Boost error correction level",
            default_value = "true",
        )
    )]
    pub boost_error_correction: bool,

    #[cfg_attr(
        feature = "cli",
        arg(long, help = "Module shape",
        value_enum, default_value_t = ModuleShape::Square,
        )
    )]
    pub shape: ModuleShape,

    #[cfg_attr(
        feature = "cli",
        arg(long, help = "Finder shape",
        value_enum,
        default_value_t = FinderShape::Square,
        )
    )]
    pub finder: FinderShape,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            short,
            help = "Path to an icon image to embed in the QR code",
        )
    )]
    pub icon: Option<String>,

    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            short,
            help = "Output file path", 
            // Default to "qr_%Y%m%d_%H%M%S.png"
            default_value_t = chrono::Local::now().format("qr_%Y-%m-%d_%H:%M:%S.png").to_string()
        )
    )]
    pub output: String,
}

impl Default for ModuleShape {
    fn default() -> Self {
        Self::Square
    }
}

impl Default for FinderShape {
    fn default() -> Self {
        Self::Square
    }
}

impl Default for GradientDirection {
    fn default() -> Self {
        Self::TopLeftToBottomRight
    }
}

impl Default for QrConfig {
    fn default() -> Self {
        Self {
            quiet_zone: 4,
            max_version: None,
            ecl: qrgen::QrCodeEcc::Medium,
            mask: None,
            foreground: vec!["#000000".to_string()],
            background: "#FFFFFF".to_string(),
            gradient_direction: GradientDirection::default(),
            ppm: 20,
            boost_error_correction: true,
            shape: ModuleShape::default(),
            finder: FinderShape::default(),
            icon: None,
            output: String::new(),
        }
    }
}
