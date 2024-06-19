// src/converter.rs
use pulldown_cmark::{html, Options, Parser};

pub fn convert_markdown_to_html(input: &str) -> String {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn convert_file_to_html(file_path: &str) -> Result<String, std::io::Error> {
    if !std::path::Path::new(file_path).exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found"));
    }

    let markdown = std::fs::read_to_string(file_path)?;
    Ok(convert_markdown_to_html(&markdown))
}