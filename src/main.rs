mod database;
mod helpers;
mod utils;

use helpers::run;

fn main() {
    match run() {
        Err(err) => eprintln!("smart-review: {}", err),
        Ok(opt_message) => match opt_message {
            Some(message) => println!("smart-review: {}", message),
            None => (),
        },
    }
}
