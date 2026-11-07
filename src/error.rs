use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("input is not UTF-8: {0}")]
    Encoding(#[from] std::string::FromUtf8Error),
    #[error("cannot parse resource reference: {0}")]
    Parsing(String),
    #[error("resource escapes input root: {0}")]
    OutsideRoot(PathBuf),
    #[error("unsupported image type: {0}")]
    ImageType(PathBuf),
    #[error("EPUB archive error: {0}")]
    Archive(#[from] zip::result::ZipError),
    #[error("clipboard failed: {0}")]
    Clipboard(String),
    #[error("external command failed: {0}")]
    Command(String),
}
pub type Result<T> = std::result::Result<T, Error>;
