use crate::{
    Converter, Options,
    error::{Error, Result},
};
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

/// `engine` is a Chromium-compatible executable, never a shell command.
pub fn write(
    markdown: &str,
    options: &Options,
    engine: &Path,
    output: &mut dyn Write,
) -> Result<()> {
    let directory = tempfile::tempdir()?;
    let html = directory.path().join("document.html");
    let pdf = directory.path().join("document.pdf");
    let mut settings = options.clone();
    settings.fragment = false;
    settings.embed_resources = true;
    super::html::Html.convert(markdown, &settings, &mut std::fs::File::create(&html)?)?;
    let status = Command::new(engine)
        .args(["--headless", "--disable-gpu", "--no-pdf-header-footer"])
        .arg(format!("--print-to-pdf={}", pdf.display()))
        .arg(&html)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| {
            Error::Command(format!(
                "cannot launch PDF engine {}: {e}; set --pdf-engine to a Chromium executable",
                engine.display()
            ))
        })?;
    if !status.success() {
        return Err(Error::Command(format!("PDF engine exited with {status}")));
    }
    std::io::copy(&mut std::fs::File::open(pdf)?, output)?;
    Ok(())
}
