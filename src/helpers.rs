use crate::{
    database::{get_all_items, get_item, init, insert_item, remove_item, update_item},
    utils::Item,
};
use anyhow::{Result, anyhow, bail};

pub fn get_stdin_content() -> Result<String, std::io::Error> {
    let mut content = String::new();
    std::io::stdin().read_line(&mut content)?;
    content.pop();

    // Additional pop for removing CR (ASCII 0D), which is additionaly sent on Windows when pressing Enter
    #[cfg(windows)]
    content.pop();

    Ok(content)
}

pub fn run() -> Result<Option<String>> {
    let arguments: Vec<String> = std::env::args().collect();
    let main_argument = arguments
        .get(1)
        .ok_or_else(|| anyhow!("must enter an argument"))?;
    let extra_argument = arguments.get(2);

    let mut message: Option<String> = None;
    let mut conn = init()?;
    match main_argument.as_str() {
        "new" => {
            println!("enter the item's title:");
            let item_title = get_stdin_content()?;

            println!("enter the item's description (Enter to skip):");
            let input = get_stdin_content()?;
            let item_description = match input.len() {
                0 => None,
                _ => Some(input),
            };

            let item = Item::new(item_title, item_description);
            insert_item(&mut conn, &item)?;
            message = Some(String::from("item inserted successfully"));
        }
        "review" => match extra_argument {
            Some(pattern) => {
                let mut item = get_item(&mut conn, pattern)?;
                println!("enter the quality of the review (in the range 1 to 5):");
                match get_stdin_content()?.parse::<u8>() {
                    Ok(grade) => {
                        if grade < 1 || grade > 5 {
                            bail!("must enter a valid number");
                        }
                        item.review(grade);
                    }
                    Err(error) => bail!("failed to parse the value into a number: {error}"),
                }
                update_item(&mut conn, item)?;
                message = Some(String::from("item reviewed successfully"))
            }
            None => bail!("must enter the title or ID of an item"),
        },
        "remove" => match extra_argument {
            Some(pattern) => {
                remove_item(&mut conn, pattern)?;
                message = Some(String::from("item removed successfully"));
            }
            None => bail!("must provide the title or ID of an item"),
        },
        "show" => match extra_argument {
            Some(pattern) => {
                let item = get_item(&mut conn, pattern)?;
                println!("{}", item);
            }
            None => {
                let query = get_all_items(&mut conn)?;
                match query {
                    Some(mut items) => {
                        items.sort_by(|a, b| a.review_date.cmp(&b.review_date));
                        for item in &items {
                            println!("{}", item);
                            print!("\n");
                        }
                    }
                    None => bail!("there are no items to print"),
                }
            }
        },
        _ => bail!("\"{main_argument}\" is not a valid argument"),
    }

    Ok(message)
}
