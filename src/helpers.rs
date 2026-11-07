pub fn get_stdin_content() -> Result<String, std::io::Error> {
    let mut content = String::new();
    std::io::stdin().read_line(&mut content)?;
    content.pop();

    // Additional pop for removing CR (ASCII 0D), which is additionaly sent on Windows when pressing Enter
    #[cfg(windows)]
    content.pop();

    Ok(content)
}
