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
}

impl Config {
    pub fn load() -> Self {
        Self::parse()
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
