// src/epub.rs
use epub_builder::{EpubBuilder, EpubContent, ZipLibrary, Result as EpubResult};
use log::info;

pub fn convert_to_epub(html: &str, output_file_path: &str, author: &str, title: &str) -> EpubResult<()> {
    let mut epub = EpubBuilder::new(ZipLibrary::new()?)?;

    epub.metadata("author", author)?;
    epub.metadata("title", title)?;

    epub.add_content(EpubContent::new("chapter1.html", html.as_bytes())
        .title("Chapter 1"))?;

    epub.generate(&mut std::fs::File::create(output_file_path)?)?;
    info!("EPUB file created successfully at {}", output_file_path);
    Ok(())
}