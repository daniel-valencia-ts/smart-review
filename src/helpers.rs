use crate::{
    database::{get_item, init, insert_item, remove_item, update_item},
    utils::Item,
};

pub fn get_stdin_content() -> Result<String, std::io::Error> {
    let mut content = String::new();
    std::io::stdin().read_line(&mut content)?;
    content.pop();

    // Additional pop for removing CR (ASCII 0D), which is additionaly sent on Windows when pressing Enter
    #[cfg(windows)]
    content.pop();

    Ok(content)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = std::env::args().collect();
    let main_argument;
    match arguments.get(1) {
        Some(arg) => main_argument = arg,
        None => return Err("No argument was provided".into()),
    }
    let extra_argument = arguments.get(2);

    let mut conn = init()?;
    match main_argument.as_str() {
        "new" => {
            println!("Enter the item's title:");
            let item_title = get_stdin_content()?;

            println!("Enter the item's description (Enter to skip):");
            let input = get_stdin_content()?;
            let item_description = match input.len() {
                0 => None,
                _ => Some(input),
            };

            let item = Item::new(item_title, item_description);
            insert_item(&mut conn, &item)?;
        }
        "review" => match extra_argument {
            Some(pattern) => {
                let mut item = get_item(&mut conn, pattern)?;
                println!("Enter the quality of the review (in the range 1 to 5):");
                match get_stdin_content()?.parse::<u8>() {
                    Ok(grade) => {
                        if grade < 1 || grade > 5 {
                            return Err("Must enter a number between 1 and 5 (inclusive)".into());
                        }
                        item.review(grade);
                    }
                    Err(error) => return Err(error.into()),
                }
                update_item(&mut conn, item)?;
            }
            None => return Err("Must provide either the item's title or ID to review it.".into()),
        },
        "remove" => match extra_argument {
            Some(pattern) => {
                remove_item(&mut conn, pattern)?;
            }
            None => {
                return Err(
                    "Must provide either the item's title or ID to be able to remove it".into(),
                );
            }
        },
        "show" => match extra_argument {
            Some(pattern) => {
                let item = get_item(&mut conn, pattern)?;
                println!("{}", item);
            }
            None => return Err("Enter the ID or title or the note to be removed".into()),
        },
        _ => return Err("Invalid arguments".into()),
    }

    Ok(())
}
