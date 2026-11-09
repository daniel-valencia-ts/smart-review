use std::io::{Write, stdout};

use crate::{
    database::{get_all_items, get_item, init, insert_item, remove_item, update_item},
    utils::Item,
};
use anyhow::{Context, Result, anyhow, bail};
use crossterm::{
    cursor::MoveToColumn,
    event::read,
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

const HELP_MESSAGE: &str = r#"usage: "sr" or "smart-review" [option] [argument]
options:

        new: adds a new item with a title and an optional description.
            takes no argument.

        review: reviews an item and schedules the next review date.
            takes as an argument the ID or title of the item.

        remove: permanently deletes the target item.
            takes as an argument its ID or title.

        show: displays the target item or all of them if no argument is given.
            optionally takes the ID or title of a target item to solely show it.

arguments longer than a single word must be enclosed within quotes.
"#;

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
    let option = arguments
        .get(1)
        .ok_or_else(|| anyhow!("must enter an argument"))?;
    let argument = arguments.get(2);

    let mut message: Option<String> = None;
    let mut conn = init()?;
    match option.as_str() {
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
        "review" => match argument {
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
        "remove" => match argument {
            Some(pattern) => {
                remove_item(&mut conn, pattern)?;
                message = Some(String::from("item removed successfully"));
            }
            None => bail!("must provide the title or ID of an item"),
        },
        "show" => match argument {
            Some(pattern) => {
                let item = get_item(&mut conn, pattern)?;
                println!("{}", item);
            }
            None => {
                let query = get_all_items(&mut conn)?;
                match query {
                    Some(mut items) => {
                        items.sort_by(|a, b| a.review_date.cmp(&b.review_date));
                        let mut items_as_iter = items.iter();
                        let instructions =
                            String::from("\"arrow down\" for printing more items, \"q\" to exit");

                        println!("{}", items_as_iter.next().unwrap());
                        print!("\n{}", instructions);
                        std::io::stdout()
                            .flush()
                            .context("failed to flush to stdout")?; // Ensure that the instructions are actually printed

                        enable_raw_mode().context("failed to enter raw mode")?;
                        while let Ok(event) = read() {
                            let Some(event) = event.as_key_press_event() else {
                                continue;
                            };
                            execute!(
                                &mut stdout(),
                                Clear(ClearType::CurrentLine),
                                MoveToColumn(0)
                            )?;
                            if event.code.is_down() || event.code.is_char('s') {
                                match items_as_iter.next() {
                                    Some(item) => {
                                        let s = item.to_string().replace('\n', "\r\n");
                                        print!("\r\n{}\r\n", s);
                                        print!("\r\n{}", instructions);
                                        std::io::stdout()
                                            .flush()
                                            .context("failed to flush to stdout")?;
                                    }
                                    None => break,
                                }
                            } else if event.code.is_esc() || event.code.is_char('q') {
                                break;
                            }
                        }

                        disable_raw_mode().context("failed to disable raw mode")?
                    }
                    None => bail!("there are no items to print"),
                }
            }
        },
        "help" => print!("{HELP_MESSAGE}"),
        _ => bail!("\"{option}\" is not a valid argument, use the \"help\" for more info"),
    }

    Ok(message)
}
