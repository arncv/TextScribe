use std::fs;
use pulldown_cmark::{html, Options, Parser};
use clap::{Arg, App};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use webbrowser;

fn convert_markdown_to_html(input: &str) -> String {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn convert_file_to_html(file_path: &str) -> Result<String, std::io::Error> {
    let markdown = fs::read_to_string(file_path)?;
    Ok(convert_markdown_to_html(&markdown))
}

fn save_html_to_file(html: &str, output_file_path: &str) -> Result<(), std::io::Error> {
    fs::write(output_file_path, html)
}

fn get_theme_css(theme: &str) -> &str {
    match theme {
        "dark" => "<style>body { background-color: black; color: white; }</style>",
        "light" => "<style>body { background-color: white; color: black; }</style>",
        _ => "", // default theme
    }
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
             .required(false)
             .index(2))
        .arg(Arg::with_name("theme")
             .help("CSS theme for the output. Available: default, dark, light")
             .takes_value(true)
             .default_value("default"))
        .arg(Arg::with_name("clipboard")
             .help("Output the generated HTML directly to the clipboard")
             .short("c")
             .long("clipboard")
             .takes_value(false))
        .arg(Arg::with_name("browser")
             .help("Preview the generated HTML in the default web browser")
             .short("b")
             .long("browser")
             .takes_value(false))
        .get_matches();

    let input_file_path = matches.value_of("input").expect("Failed to get input file path");
    let output_file_path = matches.value_of("output");
    let theme = matches.value_of("theme").expect("Failed to get theme");
    let use_clipboard = matches.is_present("clipboard");
    let preview_in_browser = matches.is_present("browser");

    let css = get_theme_css(theme);

    match convert_file_to_html(input_file_path) {
        Ok(mut html) => {
            html.insert_str(0, css); // Prepend the CSS to the HTML output
            
            if use_clipboard {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(html.clone()).expect("Failed to copy to clipboard");
                println!("HTML copied to clipboard!");
            } 
            
            if let Some(output_path) = output_file_path {
                if let Err(err) = save_html_to_file(&html, output_path) {
                    eprintln!("Error saving HTML to file: {}", err);
                } else {
                    println!("Conversion successful. HTML saved to {}", output_path);
                    if preview_in_browser {
                        if webbrowser::open(output_path).is_err() {
                            eprintln!("Failed to open the HTML in the default browser.");
                        }
                    }
                }
            } else if !use_clipboard {
                eprintln!("Please specify an output file or use the --clipboard or --browser option.");
            }
        },
        Err(err) => eprintln!("Error converting file to HTML: {}", err),
    }
}
