mod database;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = std::env::args().collect();
    let main_argument: &String;
    match arguments.get(1) {
        Some(argument) => main_argument = argument,
        None => return Err("Invalid arguments".into()),
    }

    match main_argument.as_str() {
        _ => ()
    }

    Ok(())
}
