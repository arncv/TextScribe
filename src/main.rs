// src/main.rs
mod converter;
mod highlighting;
mod image;
mod cli;
mod epub;
mod utils;


use std::time::Instant;
use std::path::Path;

fn main() {
    let start_time = Instant::now();

    let matches = cli::parse_args();

    if let Err(e) = run(matches) {
        eprintln!("Application error: {}", e);
    }

    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn run(matches: clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_file_path = matches.value_of("input").expect("Failed to get input file path");
    let theme = matches.value_of("theme").expect("Failed to get theme");
    let use_clipboard = matches.is_present("clipboard");
    let preview_in_browser = matches.is_present("browser");
    let css_file_path = matches.value_of("css");
    let output_file_path = matches.value_of("output");

    let css = if let Some(path) = css_file_path {
        match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Warning: Failed to read CSS file {}: {}", path, err);
                utils::get_theme_css(theme).to_string()
            }
        }
    } else {
        utils::get_theme_css(theme).to_string()
    };

    let mut html = converter::convert_file_to_html(input_file_path)?;
    html.insert_str(0, &css); // Prepend the CSS to the HTML output

    let base_path = Path::new(input_file_path).parent().unwrap_or_else(|| Path::new("."));
    image::embed_images_as_base64(&mut html, base_path);

    // Apply syntax highlighting
    html = highlighting::apply_syntax_highlighting(&html);

    if use_clipboard {
        utils::copy_to_clipboard(&html)?;
    }

    if matches.is_present("epub") {
        let output_epub_path = "output.epub";
        epub::convert_to_epub(&html, output_epub_path)?;
    }

    if matches.is_present("pdf") {
        utils::export_html_to_pdf(&html, "output.pdf")?;
    }

    if let Some(output_path) = output_file_path {
        utils::save_html_to_file(&html, output_path)?;
        println!("Conversion successful. HTML saved to {}", output_path);
        if preview_in_browser {
            if webbrowser::open(output_path).is_err() {
                eprintln!("Failed to open the HTML in the default browser.");
            }
        }
    } else if !use_clipboard {
        eprintln!("Please specify an output file or use the --clipboard or --browser option.");
    }

    Ok(())
}