use super::html::escape;
use crate::{Options, error::Result};
use pulldown_cmark::{CowStr, Event, Parser, Tag};
use std::io::{Seek, Write};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

/// A stored ZIP streams entries without buffering the archive in memory.
pub fn write<W: Write + Seek>(markdown: &str, options: &Options, output: W) -> Result<()> {
    if markdown.chars().chain(options.title.chars()).any(|c|
        !matches!(c, '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}'..='\u{d7ff}' | '\u{e000}'..='\u{fffd}' | '\u{10000}'..='\u{10ffff}')) {
        return Err(crate::error::Error::Parsing("EPUB text contains a character forbidden by XML 1.0".into()));
    }
    let mut resources = Vec::new();
    let events = Parser::new_ext(markdown, options.parser)
        .map(|event| match event {
            Event::Html(text) | Event::InlineHtml(text) => Ok(Event::Text(text)),
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let path = super::html::resolve_image(&options.resource_root, &dest_url)?;
                let mime = super::html::image_mime(&path)?;
                let name = format!("image-{}", resources.len());
                resources.push((name.clone(), mime, path));
                Ok(Event::Start(Tag::Image {
                    link_type,
                    dest_url: CowStr::from(name),
                    title,
                    id,
                }))
            }
            event => Ok(event),
        })
        .collect::<Result<Vec<_>>>()?;
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, events.into_iter());
    let manifest: String = resources
        .iter()
        .enumerate()
        .map(|(i, (name, mime, _))| {
            format!(r#"<item id="image-{i}" href="{name}" media-type="{mime}"/>"#)
        })
        .collect();
    let mut zip = ZipWriter::new(output);
    let stored = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    zip.start_file("mimetype", stored)?;
    zip.write_all(b"application/epub+zip")?;
    zip.start_file("META-INF/container.xml", stored)?;
    zip.write_all(br#"<?xml version="1.0"?><container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="EPUB/package.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#)?;
    let title = escape(&options.title);
    zip.start_file("EPUB/package.opf", stored)?;
    write!(
        zip,
        r#"<?xml version="1.0" encoding="UTF-8"?><package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:identifier id="id">urn:textscribe:document</dc:identifier><dc:title>{title}</dc:title><dc:language>en</dc:language><meta property="dcterms:modified">2000-01-01T00:00:00Z</meta></metadata><manifest><item id="content" href="content.xhtml" media-type="application/xhtml+xml"/><item id="nav" href="nav.xhtml" media-type="application/xhtml+xml" properties="nav"/>{manifest}</manifest><spine><itemref idref="content"/></spine></package>"#
    )?;
    zip.start_file("EPUB/nav.xhtml", stored)?;
    write!(
        zip,
        r#"<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops"><head><title>{title}</title></head><body><nav epub:type="toc"><ol><li><a href="content.xhtml">{title}</a></li></ol></nav></body></html>"#
    )?;
    zip.start_file("EPUB/content.xhtml", stored)?;
    write!(
        zip,
        r#"<html xmlns="http://www.w3.org/1999/xhtml"><head><title>{title}</title></head><body>{body}</body></html>"#
    )?;
    for (name, _, path) in resources {
        zip.start_file(format!("EPUB/{name}"), stored)?;
        std::io::copy(&mut std::fs::File::open(path)?, &mut zip)?;
    }
    zip.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packaged_images_and_escaped_html() -> Result<()> {
        let root = tempfile::tempdir()?;
        std::fs::write(root.path().join("a.png"), [1, 2, 3])?;
        let options = Options {
            resource_root: root.path().into(),
            ..Options::default()
        };
        let mut archive = std::io::Cursor::new(Vec::new());
        write("![alt](a.png)\n\n<bad>\n", &options, &mut archive)?;
        let mut zip = zip::ZipArchive::new(archive)?;
        use std::io::Read;
        let mut content = String::new();
        zip.by_name("EPUB/content.xhtml")?
            .read_to_string(&mut content)?;
        assert!(content.contains("src=\"image-0\""));
        assert!(content.contains("&lt;bad&gt;"));
        let mut bytes = Vec::new();
        zip.by_name("EPUB/image-0")?.read_to_end(&mut bytes)?;
        assert_eq!(bytes, [1, 2, 3]);
        Ok(())
    }
}
