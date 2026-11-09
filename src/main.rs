mod database;
mod helpers;
mod utils;

use colored::Colorize;
use helpers::run;

fn main() {
    match run() {
        Err(err) => eprintln!("smart-review: {}", err.to_string().red().bold()),
        Ok(opt_message) => match opt_message {
            Some(message) => println!("smart-review: {}", message.green().bold()),
            None => (),
        },
    }
}
