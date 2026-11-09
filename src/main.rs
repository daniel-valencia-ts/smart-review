mod database;
mod helpers;
mod utils;

use colored::Colorize;
use crossterm::terminal::disable_raw_mode;
use helpers::run;

fn main() {
    match run() {
        Err(err) => eprintln!("smart-review: {}", err.to_string().red().bold()),
        Ok(opt_message) => match opt_message {
            Some(message) => println!("smart-review: {}", message.green().bold()),
            None => (),
        },
    }
    if let Err(_) = disable_raw_mode() {
        eprintln!(
            "smart-review: {}",
            "couldn't disable raw mode-".to_string().red().bold()
        );
    }
}
