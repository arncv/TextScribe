use std::fs;
use std::path::Path;
use pulldown_cmark::{html, Options, Parser};
use clap::{Arg, App};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use webbrowser;
use base64;
use image::{ImageOutputFormat, ImageFormat, io::Reader as ImageReader};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet};
use syntect::html::highlighted_html_for_string;
use std::process::Command;
use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::ZipLibrary;
use epub_builder::Result as EpubResult;



// Add this function to handle syntax highlighting
fn apply_syntax_highlighting(html: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let theme = &theme_set.themes["base16-ocean.dark"];
    let mut highlighted_html = String::new();

    for line in html.lines() {
        if line.starts_with("<pre><code>") && line.ends_with("</code></pre>") {
            let code = &line[11..line.len() - 12];
            let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
            let highlighted = highlighted_html_for_string(code, &syntax_set, syntax, theme);
            highlighted_html.push_str(&format!("<pre><code>{}</code></pre>", highlighted));
        } else {
            highlighted_html.push_str(line);
        }
    }

    highlighted_html
}

fn optimize_image(img_path: &Path) -> Result<(Vec<u8>, ImageFormat), image::ImageError> {
    let img = image::open(img_path)?;
    let mut optimized_img = Vec::new();

    // Determine the image format
    let format = ImageReader::open(img_path)?.format().unwrap_or(ImageFormat::Png); // Default to PNG if format detection fails

    match format {
        ImageFormat::Png => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Png)?;
        },
        ImageFormat::Jpeg => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Jpeg(80))?; // 80 is the quality setting
        },
        ImageFormat::Gif => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Gif)?;
        },
        // Add other formats as needed
        _ => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Png)?;
        }
    }
    Ok((optimized_img, format))
}
fn convert_to_epub(html: &str, output_file_path: &str) -> EpubResult<()> {
    let mut epub = EpubBuilder::new(ZipLibrary::new()?)?;

    epub.metadata("author", "Your Name")?;
    epub.metadata("title", "Your Title")?;

    epub.add_content(EpubContent::new("chapter1.html", html.as_bytes())
        .title("Chapter 1"))?;

    epub.generate(&mut std::fs::File::create(output_file_path)?)?;

    Ok(())
}



// This function is used to optimize the image

fn get_data_url_prefix(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "data:image/png;base64,",
        ImageFormat::Jpeg => "data:image/jpeg;base64,",
        ImageFormat::Gif => "data:image/gif;base64,",
        // Add other formats as needed
        _ => "data:image/png;base64,", // default to PNG
    }
}

// This function is used to get the prefix of the data url

fn embed_images_as_base64(html_output: &mut String, base_path: &Path) {
    let img_tag_pattern = "<img src=\"";
    let mut index = 0;

    while let Some(start) = html_output[index..].find(img_tag_pattern) {
        let start = start + index;
        let end = html_output[start..].find("\"").unwrap() + start + img_tag_pattern.len();
        let img_path_str = &html_output[start + img_tag_pattern.len()..end];
        let img_path = base_path.join(img_path_str);

        if fs::read(img_path.clone()).is_ok() {
            let (optimized_data, img_format) = match optimize_image(&img_path) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Warning: Failed to optimize image {}: {}", img_path.display(), e);
                    (fs::read(img_path.clone()).unwrap(), ImageFormat::Png) // default to PNG if optimization fails
                }
            };
            
            let encoded = base64::encode(&optimized_data);
            let prefix = get_data_url_prefix(img_format);
            let data_url = format!("{}{}", prefix, encoded);
            html_output.replace_range((start + img_tag_pattern.len())..end, &data_url);
        }

        index = end + 1;
    }
}

// Embeds the image as base64 

fn convert_markdown_to_html(input: &str) -> String {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
// Main function

fn convert_file_to_html(file_path: &str) -> Result<String, std::io::Error> {
    if !Path::new(file_path).exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found"));
    }

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
    env_logger::init();
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
        .arg(Arg::with_name("pdf")
             .help("Export the generated HTML to a PDF file")
             .short("p")
             .long("pdf")
             .takes_value(false))
        .arg(Arg::with_name("browser")
             .help("Preview the generated HTML in the default web browser")
             .short("b")
             .long("browser")
             .takes_value(false))
        .arg(Arg::with_name("css")
             .help("Path to an external CSS file to include in the output HTML")
             .takes_value(true)
             .long("css"))
        .arg(Arg::with_name("pdf_name")
             .help("Specify the output PDF file name")
             .takes_value(true)
             .short("n")
             .long("pdf-name"))
        .arg(Arg::with_name("epub")
             .help("Export the generated HTML to an EPUB file")
             .short("e")
             .long("epub")
             .takes_value(false))
        .arg(Arg::with_name("verbose")
         .help("Enable verbose output")
         .short("v")
         .long("verbose")
         .takes_value(false))
        .get_matches();

    let verbose_mode = matches.is_present("verbose");
    let input_file_path = matches.value_of("input").expect("Failed to get input file path");
    let output_pdf_path = matches.value_of("pdf_name").unwrap_or("output.pdf");
    let theme = matches.value_of("theme").expect("Failed to get theme");
    let use_clipboard = matches.is_present("clipboard");
    let preview_in_browser = matches.is_present("browser");
    let css_file_path = matches.value_of("css");
    let output_file_path = matches.value_of("output");

    let export_to_pdf = matches.is_present("pdf");    
    let css = if let Some(path) = css_file_path {
        match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Warning: Failed to read CSS file {}: {}", path, err);
                get_theme_css(theme).to_string()
            }
        }
    } else {
        get_theme_css(theme).to_string()
    };
    

    match convert_file_to_html(input_file_path) {
        Ok(mut html) => {
            html.insert_str(0, &css); // Prepend the CSS to the HTML output
            if verbose_mode {
                log::info!("Markdown file converted to HTML.");
            }
            

            
            let base_path = Path::new(input_file_path).parent().unwrap_or_else(|| Path::new("."));
            embed_images_as_base64(&mut html, base_path);

            // Apply syntax highlighting
            html = apply_syntax_highlighting(&html);
            
            if use_clipboard {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(html.clone()).expect("Failed to copy to clipboard");
                if verbose_mode {
                    println!("HTML copied to clipboard!");
                }
            }
            if matches.is_present("epub") {
                let output_epub_path = "output.epub";
                match convert_to_epub(&html, output_epub_path) {
                    Ok(_) => println!("Successfully exported to EPUB: {}", output_epub_path),
                    Err(err) => eprintln!("Failed to export to EPUB: {}", err),
                }
            }
            
            

            if export_to_pdf {
                let output = Command::new("wkhtmltopdf")
                    .arg("example.html") 
                    .arg(output_pdf_path)
                    .output()
                    .expect("Failed to execute wkhtmltopdf");
        
                if output.status.success() {
                    println!("Successfully exported to PDF: {}", output_pdf_path);
                } else {
                    eprintln!("Failed to export to PDF");
                    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
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
        Err(err) => {
            eprintln!("Error converting file to HTML: {}", err);

            if verbose_mode {
                eprintln!("Verbose: Conversion failed due to: {:?}", err.kind());
            }
        }
    }
}

