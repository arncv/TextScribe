use crate::{
    Converter, Options, Theme,
    error::{Error, Result},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use pulldown_cmark::{CowStr, Event, Parser, Tag};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

pub struct Html;
pub const CSS: &str = include_str!("theme.css");
pub fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
/// Canonicalization also rejects symlinks that point outside the root.
pub fn resolve_image(root: &Path, reference: &str) -> Result<PathBuf> {
    if reference.contains(['?', '#', '%']) {
        return Err(Error::Parsing(format!(
            "image path must be a literal local path: {reference}"
        )));
    }
    let root = root.canonicalize()?;
    let path = root.join(reference).canonicalize()?;
    if !path.starts_with(&root) {
        return Err(Error::OutsideRoot(path));
    }
    Ok(path)
}
pub fn image_data(root: &Path, reference: &str) -> Result<String> {
    let path = resolve_image(root, reference)?;
    let mime = image_mime(&path)?;
    Ok(format!(
        "data:{mime};base64,{}",
        STANDARD.encode(fs::read(path)?)
    ))
}
pub fn image_mime(path: &Path) -> Result<&'static str> {
    let mime = match path
        .extension()
        .and_then(|x| x.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("avif") => "image/avif",
        _ => return Err(Error::ImageType(path.to_path_buf())),
    };
    Ok(mime)
}
pub fn body(markdown: &str, options: &Options) -> Result<String> {
    let events = Parser::new_ext(markdown, options.parser).map(|event| {
        if let Event::Start(Tag::Image { link_type, dest_url, title, id }) = event {
            let dest_url = if options.embed_resources && !dest_url.starts_with("data:") {
                if dest_url.contains("://") || dest_url.starts_with("//") {
                    return Err(Error::Parsing(format!("remote image cannot be embedded: {dest_url}; download it into the input directory")));
                }
                CowStr::from(image_data(&options.resource_root, &dest_url)?)
            } else { dest_url };
            Ok(Event::Start(Tag::Image { link_type, dest_url, title, id }))
        } else { Ok(event) }
    }).collect::<Result<Vec<_>>>()?;
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    Ok(html)
}
impl Converter for Html {
    fn convert(&self, markdown: &str, options: &Options, output: &mut dyn Write) -> Result<()> {
        let body = body(markdown, options)?;
        if options.fragment {
            output.write_all(body.as_bytes())?;
        } else {
            let theme = match options.theme {
                Theme::Auto => "auto",
                Theme::Light => "light",
                Theme::Dark => "dark",
            };
            write!(
                output,
                "<!doctype html>\n<html lang=\"en\" data-theme=\"{theme}\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>{}</title><style>{CSS}</style></head><body><main>{body}</main></body></html>\n",
                escape(&options.title)
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing_and_theme() -> Result<()> {
        let options = Options {
            title: "<Title>".into(),
            theme: Theme::Dark,
            ..Options::default()
        };
        let mut output = Vec::new();
        Html.convert("# Hello\n\n- [x] done\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n[^a]: note\n\nref[^a]\n\n```rust\n<&\n```", &options, &mut output)?;
        let result = String::from_utf8(output)?;
        for expected in [
            "data-theme=\"dark\"",
            "&lt;Title&gt;",
            "<style>",
            "<table>",
            "checkbox",
            "footnote",
            "language-rust",
            "&lt;&amp;",
        ] {
            assert!(result.contains(expected), "missing {expected}");
        }
        assert_eq!(body("", &options)?, "");
        Ok(())
    }
    #[test]
    fn images_and_boundaries() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("root");
        fs::create_dir(&root)?;
        fs::write(root.join("test.png"), [1, 2, 3])?;
        fs::write(dir.path().join("secret.png"), [4])?;
        assert_eq!(image_data(&root, "test.png")?, "data:image/png;base64,AQID");
        assert!(matches!(
            resolve_image(&root, "../secret.png"),
            Err(Error::OutsideRoot(_))
        ));
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(dir.path().join("secret.png"), root.join("link.png"))?;
            assert!(matches!(
                resolve_image(&root, "link.png"),
                Err(Error::OutsideRoot(_))
            ));
        }
        let options = Options {
            resource_root: root,
            embed_resources: true,
            ..Options::default()
        };
        assert!(body("![a](test.png)", &options)?.contains("data:image/png;base64,AQID"));
        assert!(body("![a](https://example.com/a.png)", &options).is_err());
        Ok(())
    }
}
