use clap::Parser;
use std::str::FromStr;

/// Blink
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Config {
    /// The size of blink-bar
    ///
    /// Either in pixel or a percentage of the screen size, mixed values are not supported.
    /// - 0.66,0.066
    /// - 800px,80px
    #[clap(short, long, default_value = "0.66,0.066")]
    pub size: WindowSize,

    /// The position of blink-bar
    ///
    /// Either in pixel or a percentage of the screen size, mixed values are not supported.
    /// - 0.5,0.33
    /// - 960px,300px
    #[clap(short, long, default_value = "0.5,0.33")]
    pub location: WindowPosition,

    /// The font size, in pixels
    ///
    /// Must end in 'px'
    /// - 24px
    #[clap(long, default_value = "18px")]
    pub font_size: FontSize,

    /// The font family to use
    #[clap(long)]
    pub font_family: Option<String>,

    /// The background color
    ///
    /// A color string has the format '#RRGGBB' or '#RRGGBBAA'
    #[clap(long, default_value = "#35313bee")]
    pub background_color: Color,

    /// The text color
    ///
    /// A color string has the format '#RRGGBB' or '#RRGGBBAA'
    #[clap(long, default_value = "#f8f8f2ff")]
    pub text_color: Color,
}

impl Config {
    pub fn load() -> Self {
        Self::parse()
    }
}

/// A color string
///
/// Has the format '#RRGGBB' or '#RRGGBBAA',
/// where R, G, B, and A are hexadecimal digits.
#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');
        let len = s.len();
        if len != 6 && len != 8 {
            return Err("Expected format: #RRGGBB or #RRGGBBAA".to_string());
        }

        let red = u8::from_str_radix(&s[0..2], 16).map_err(|e| e.to_string())?;
        let green = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
        let blue = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
        let alpha = if len == 8 {
            u8::from_str_radix(&s[6..8], 16).map_err(|e| e.to_string())?
        } else {
            0xff
        };

        Ok(Color {
            red,
            green,
            blue,
            alpha,
        })
    }
}

/// The font size, in pixels
#[derive(Debug, Clone)]
pub struct FontSize(pub u32);

impl FromStr for FontSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("px") {
            let size = s
                .trim_end_matches("px")
                .parse::<u32>()
                .map_err(|e| e.to_string())?;
            Ok(FontSize(size))
        } else {
            Err("Expected format: <size>px".to_string())
        }
    }
}

/// The position of the window, either in pixels or as a percentage of the screen size.
pub type WindowPosition = WindowSize;

/// The size of the window, either in pixels or as a percentage of the screen size.
#[derive(Debug, Clone)]
pub enum WindowSize {
    Percent(f32, f32),
    Pixels(u32, u32),
}

impl FromStr for WindowSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Expected format: <width>,<height>".to_string());
        }

        if parts[0].ends_with("px") {
            let width = parts[0]
                .trim_end_matches("px")
                .parse::<u32>()
                .map_err(|e| e.to_string())?;

            let height = parts[1]
                .trim_end_matches("px")
                .parse::<u32>()
                .map_err(|e| e.to_string())?;

            Ok(WindowSize::Pixels(width, height))
        } else {
            let width = parts[0].parse::<f32>().map_err(|e| e.to_string())?;

            let height = parts[1].parse::<f32>().map_err(|e| e.to_string())?;

            Ok(WindowSize::Percent(width, height))
        }
    }
}
