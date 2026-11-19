use std::io::{Write, stdout};

use crate::{
    database::{get_all_items, get_item, init, insert_item, remove_item, update_item},
    utils::{Cli, Item, Operations},
};
use anyhow::{Context, Result, bail};
use clap::Parser;
use crossterm::{
    cursor::MoveToColumn,
    event::read,
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

pub fn run() -> Result<Option<String>> {
    let args = Cli::parse();

    let mut message: Option<String> = None;
    let mut conn = init()?;

    match args.operations {
        Operations::New { title, description } => {
            let item = Item::new(title, description);
            insert_item(&mut conn, &item)?;
            message = Some(String::from("item inserted successfully"));
        }
        Operations::Review {
            raw_pattern,
            review_grade,
        } => {
            let pattern = match u32::from_str_radix(&raw_pattern, 16) {
                Ok(val) => val.to_string(),
                Err(_) => raw_pattern,
            };
            let mut item = get_item(&mut conn, &pattern)?;
            if review_grade > 5 {
                bail!("must enter a valid number");
            }
            item.review(review_grade);
            update_item(&mut conn, item)?;
            message = Some(String::from("item reviewed successfully"))
        }
        Operations::Remove { raw_pattern } => {
            let pattern = match u32::from_str_radix(&raw_pattern, 16) {
                Ok(val) => val.to_string(),
                Err(_) => raw_pattern,
            };
            remove_item(&mut conn, &pattern)?;
            message = Some(String::from("item removed successfully"));
        }
        Operations::Show { raw_pattern } => {
            match raw_pattern {
                Some(patt) => {
                    let pattern = match u32::from_str_radix(&patt, 16) {
                        Ok(val) => val.to_string(),
                        Err(_) => patt,
                    };
                    let item = get_item(&mut conn, &pattern)?;
                    println!("{}", item);
                }
                None => {
                    let query = get_all_items(&mut conn)?;
                    match query {
                        Some(mut items) => {
                            items.sort_by(|a, b| a.review_date.cmp(&b.review_date));
                            let mut items_as_iter = items.iter();
                            let instructions = String::from(
                                "\"arrow down\" for printing more items, \"q\" to exit",
                            );

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
            }
        }
    }

    Ok(message)
}
