// main.rs

use std::env;
use std::fs;
use std::io::{self, Read};
use pulldown_cmark::{html, Options, Parser};

fn convert_markdown_to_html(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn convert_file_to_html(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut markdown = String::new();
    file.read_to_string(&mut markdown)?;
    Ok(convert_markdown_to_html(&markdown))
}

fn save_html_to_file(html: &str, output_file_path: &str) -> Result<(), std::io::Error> {
    fs::write(output_file_path, html)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: rust-markdown-converter <input-file> <output-file>");
        return;
    }

    let input_file_path = &args[1];
    let output_file_path = &args[2];

    match convert_file_to_html(input_file_path) {
        Ok(html) => {
            match save_html_to_file(&html, output_file_path) {
                Ok(_) => println!("Conversion successful. HTML saved to {}",output_file_path),
                Err(err) => eprintln!("Error saving HTML to file: {}", err),
                }
                },
                Err(err) => eprintln!("Error converting file to HTML: {}", err),
                }
                }
