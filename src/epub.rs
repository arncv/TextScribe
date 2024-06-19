// src/epub.rs
use epub_builder::{EpubBuilder, EpubContent, ZipLibrary, Result as EpubResult};

pub fn convert_to_epub(html: &str, output_file_path: &str) -> EpubResult<()> {
    let mut epub = EpubBuilder::new(ZipLibrary::new()?)?;

    epub.metadata("author", "Your Name")?;
    epub.metadata("title", "Your Title")?;

    epub.add_content(EpubContent::new("chapter1.html", html.as_bytes())
        .title("Chapter 1"))?;

    epub.generate(&mut std::fs::File::create(output_file_path)?)?;

    Ok(())
}