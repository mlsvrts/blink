//! # Blink
//!
//! Blink is a 'go-bar'; a cross-platform pop-up bar for executing commands,
//! and triggering shortcuts.

/// Support for the 'slint' rendering backend.
mod backends;

/// Core logic for executing commands based on a command-file
mod commands;

/// Configuration
mod config;

fn core(input: String) {
    println!("Alive: {}!", input);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::load();

    backends::slint::render(&config, core)
}
