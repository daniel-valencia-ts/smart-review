mod database;
mod helpers;
mod utils;

use helpers::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()
}
