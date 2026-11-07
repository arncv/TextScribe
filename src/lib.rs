pub mod converter;
pub mod error;
pub mod io;

use pulldown_cmark::Options as ParserOptions;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    Auto,
}
#[derive(Clone, Debug)]
pub struct Options {
    pub title: String,
    pub theme: Theme,
    pub fragment: bool,
    pub embed_resources: bool,
    pub resource_root: PathBuf,
    pub parser: ParserOptions,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            title: "Document".into(),
            theme: Theme::Auto,
            fragment: false,
            embed_resources: false,
            resource_root: PathBuf::from("."),
            parser: ParserOptions::ENABLE_TABLES
                | ParserOptions::ENABLE_STRIKETHROUGH
                | ParserOptions::ENABLE_TASKLISTS
                | ParserOptions::ENABLE_FOOTNOTES,
        }
    }
}
/// Converters write to a caller-owned stream and never select a destination.
pub trait Converter {
    fn convert(
        &self,
        markdown: &str,
        options: &Options,
        output: &mut dyn std::io::Write,
    ) -> error::Result<()>;
}
