use std::fs;
use std::io::{self, Read};
use pulldown_cmark::{html, Options, Parser};
use clap::{Arg, App};

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
    let matches = App::new("Rust Markdown Converter")
        .version("1.0")
        .author("Your Name")
        .about("Converts Markdown to HTML")
        .arg(Arg::with_name("input")
             .help("Input markdown file")
             .required(true)
             .index(1))
        .arg(Arg::with_name("output")
             .help("Output HTML file")
             .required(true)
             .index(2))
        .arg(Arg::with_name("theme")
             .help("CSS theme for the output. Available: default, dark, light")
             .takes_value(true)
             .default_value("default"))
        .get_matches();

    let input_file_path = matches.value_of("input").unwrap();
    let output_file_path = matches.value_of("output").unwrap();
    let theme = matches.value_of("theme").unwrap();

    // Basic theming (for demonstration purposes)
    let css = match theme {
        "dark" => "<style>body { background-color: black; color: white; }</style>",
        "light" => "<style>body { background-color: white; color: black; }</style>",
        _ => "", // default theme
    };

    match convert_file_to_html(input_file_path) {
        Ok(mut html) => {
            html.insert_str(0, css); // Prepend the CSS to the HTML output
            match save_html_to_file(&html, output_file_path) {
                Ok(_) => println!("Conversion successful. HTML saved to {}", output_file_path),
                Err(err) => eprintln!("Error saving HTML to file: {}", err),
            }
        },
        Err(err) => eprintln!("Error converting file to HTML: {}", err),
    }
}
