use crate::core::QrCodeEcc;

#[cfg(feature = "cli")]
use clap::{Args, ValueEnum};

/// QR code module shapes.
/// These shapes determine how each module (square) of the QR code is rendered.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "batch", serde(rename_all = "kebab-case"))]
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
#[cfg_attr(feature = "batch", serde(rename_all = "kebab-case"))]
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
#[cfg_attr(feature = "batch", serde(rename_all = "kebab-case"))]
pub enum GradientDirection {
    TopToBottom,
    LeftToRight,
    TopLeftToBottomRight,
    BottomLeftToTopRight,
    Radial,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "batch", serde(rename_all = "kebab-case"))]
pub enum OutputFormat {
    /// Output as PNG image.
    Png,
    /// Output as JPEG image.
    Jpg,
    /// Output as JPEG image.
    Jpeg,
    /// Output as BMP image.
    Bmp,
    /// Output as TIFF image.
    Tiff,
    /// Output as GIF image.
    Gif,
    /// Output as ICO image.
    Ico,
    /// Output as WebP image.
    Webp,
    /// Output as SVG vector graphic.
    Svg,
    /// Output as EPS vector graphic.
    Eps,
    /// Output as PDF document.
    Pdf,
}

/// Configuration options for generating and rendering a QR code.
/// This struct holds various settings that control the appearance
/// and behavior of the generated QR code.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(Args))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "batch", serde(default))]
pub struct QrConfig {
    /// Modules of quiet zone around the QR code (0-10).
    /// A quiet zone is a margin of empty space around the QR code
    /// to improve readability by scanners.
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            default_value = "4",
            value_parser = clap::value_parser!(u32).range(0..=10),
        )
    )]
    pub quiet_zone: u32,

    /// Maximum QR code version.
    /// Limits the size of the QR code.
    /// Legal values are 1 to 40.
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            value_parser = clap::value_parser!(u8).range(1..=40)
        )
    )]
    pub max_version: Option<u8>,

    /// Error correction level.
    /// Higher levels increase redundancy but reduce data capacity.
    #[cfg_attr(feature = "cli", arg(long, default_value = "medium",))]
    pub ecl: QrCodeEcc,

    /// Mask pattern to use (0-7).
    /// If not set, it will be chosen automatically.
    /// Usually you don't need to set this.
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            value_parser = clap::value_parser!(u8).range(0..=7)
        )
    )]
    pub mask: Option<u8>,

    /// Foreground color(s). If multiple colors are provided, a gradient is created.
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            help = "Foreground color(s). If multiple colors are provided, a gradient is created.",
            default_value = "#000000",
            num_args = 1..,
        )
    )]
    #[cfg_attr(
        feature = "batch",
        serde(deserialize_with = "deserialize_string_or_vec")
    )]
    pub foreground: Vec<String>,

    /// Background color.
    #[cfg_attr(feature = "cli", arg(long, default_value = "#FFFFFF",))]
    pub background: String,

    /// Gradient direction.
    /// Determines the direction of the color gradient when multiple foreground colors are used.
    /// Only applicable if multiple foreground colors are specified.
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            value_enum,
            default_value_t = GradientDirection::TopLeftToBottomRight,
        )
    )]
    pub gradient_direction: GradientDirection,

    /// Pixels per module.
    /// Determines the size of each module (square) in the QR code in pixels.
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

    /// Whether to boost error correction level.
    #[cfg_attr(
        feature = "cli",
        arg(long, help = "Boost error correction level", default_value = "true",)
    )]
    pub boost_error_correction: bool,

    /// Module shape.
    /// Determines how each module (square) of the QR code is rendered.
    #[cfg_attr(
        feature = "cli",
        arg(long,
        value_enum, default_value_t = ModuleShape::Square,
        )
    )]
    pub shape: ModuleShape,

    /// Finder pattern shape.
    /// Determines how the position detection patterns ("eyes") of the QR code are rendered.
    #[cfg_attr(
        feature = "cli",
        arg(long,
        value_enum,
        default_value_t = FinderShape::Square,
        )
    )]
    pub finder: FinderShape,

    /// Path to an icon image to embed in the QR code.
    /// The icon will be placed at the center of the QR code.
    #[cfg_attr(feature = "cli", arg(long, short,))]
    pub icon: Option<String>,

    /// Output format.
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            value_enum,
            default_value_t = OutputFormat::Png,
        )
    )]
    pub format: OutputFormat,
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
            ecl: QrCodeEcc::Medium,
            mask: None,
            foreground: vec!["#000000".to_string()],
            background: "#FFFFFF".to_string(),
            gradient_direction: GradientDirection::default(),
            ppm: 20,
            boost_error_correction: true,
            shape: ModuleShape::default(),
            finder: FinderShape::default(),
            icon: None,
            format: OutputFormat::Png,
        }
    }
}

#[cfg(feature = "batch")]
fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::String(s) => Ok(vec![s]),
        StringOrVec::Vec(v) => Ok(v),
    }
}
