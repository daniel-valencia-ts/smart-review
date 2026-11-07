mod database;
mod utils;
mod io;

use database::{init, insert_item};
use utils::Item;
use io::get_stdin_content;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = std::env::args().collect();
    let main_argument: &String;
    match arguments.get(1) {
        Some(argument) => main_argument = argument,
        None => return Err("No argument was provided".into()),
    }


    let mut conn = init()?;
    match main_argument.as_str() {
        "new" => {
            println!("Enter the item's title:");
            let item_title = get_stdin_content()?;

            println!("Enter the item's description (Enter to skip):");
            let input = get_stdin_content()?;
            let item_description = match input.len() {
                0 => None,
                _ => Some(input)
            };

            let item = Item::new(item_title, item_description);
            insert_item(&mut conn, &item)?;
        }
        _ => return Err("Invalid arguments".into())
    }

    Ok(())
}
