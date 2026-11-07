use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use pulldown_cmark::Options as ParserOptions;
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};
use textscribe::{
    Converter, Options, Theme,
    converter::{epub, html::Html, pdf},
};

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Html,
    Epub,
    Pdf,
}
#[derive(Clone, Copy, ValueEnum)]
enum Color {
    Auto,
    Light,
    Dark,
}
#[derive(Parser)]
#[command(version, about = "Convert Markdown from a file or stdin")]
struct Args {
    /// Input file; omit or use - for stdin
    input: Option<PathBuf>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long, value_enum, default_value = "html")]
    format: Format,
    #[arg(long, value_enum, default_value = "auto")]
    theme: Color,
    #[arg(long, default_value = "Document")]
    title: String,
    #[arg(long)]
    fragment: bool,
    /// Inline local Markdown images; remote URLs are rejected
    #[arg(long, alias = "standalone")]
    embed_resources: bool,
    /// Image root for stdin (file input defaults to its parent)
    #[arg(long)]
    resource_root: Option<PathBuf>,
    #[arg(long)]
    no_tables: bool,
    #[arg(long)]
    no_strikethrough: bool,
    #[arg(long)]
    no_tasklists: bool,
    #[arg(long)]
    no_footnotes: bool,
    /// Copy HTML using wl-copy (Wayland) or xclip (X11)
    #[arg(long)]
    clipboard: bool,
    /// Open HTML with xdg-open
    #[arg(long)]
    browser: bool,
    /// Chromium-compatible executable for PDF output
    #[arg(long)]
    pdf_engine: Option<PathBuf>,
    #[arg(short, long)]
    verbose: bool,
}
fn run(args: Args) -> Result<()> {
    if (args.clipboard || args.browser || args.fragment) && !matches!(args.format, Format::Html) {
        bail!("--clipboard, --browser, and --fragment require --format html");
    }
    let root = args
        .resource_root
        .clone()
        .or_else(|| {
            args.input
                .as_deref()
                .filter(|p| *p != Path::new("-"))
                .and_then(Path::parent)
                .map(Path::to_path_buf)
        })
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| PathBuf::from("."));
    let mut options = Options {
        title: args.title,
        theme: match args.theme {
            Color::Auto => Theme::Auto,
            Color::Light => Theme::Light,
            Color::Dark => Theme::Dark,
        },
        fragment: args.fragment,
        embed_resources: args.embed_resources,
        resource_root: root,
        ..Options::default()
    };
    for (disabled, flag) in [
        (args.no_tables, ParserOptions::ENABLE_TABLES),
        (args.no_strikethrough, ParserOptions::ENABLE_STRIKETHROUGH),
        (args.no_tasklists, ParserOptions::ENABLE_TASKLISTS),
        (args.no_footnotes, ParserOptions::ENABLE_FOOTNOTES),
    ] {
        options.parser.set(flag, !disabled);
    }
    let markdown = textscribe::io::read(
        textscribe::io::input(args.input.as_deref()).context("opening input")?,
    )?;
    if args.verbose {
        eprintln!("textscribe: read {} bytes", markdown.len());
    }
    // A temporary seekable file permits EPUB output on pipes without buffering the archive.
    let mut artifact = tempfile::tempfile()?;
    match args.format {
        Format::Html => Html.convert(&markdown, &options, &mut artifact)?,
        Format::Epub => epub::write(&markdown, &options, &mut artifact)?,
        Format::Pdf => {
            let engine = args
                .pdf_engine
                .context("PDF requires --pdf-engine /path/to/chromium")?;
            pdf::write(&markdown, &options, &engine, &mut artifact)?;
        }
    }
    use std::io::{Seek, SeekFrom};
    artifact.seek(SeekFrom::Start(0))?;
    if args.output.is_some() || !(args.clipboard || args.browser) {
        let mut output =
            textscribe::io::output(args.output.as_deref()).context("opening output")?;
        io::copy(&mut artifact, &mut output)?;
        output.flush()?;
    }
    if args.clipboard {
        artifact.seek(SeekFrom::Start(0))?;
        let (program, flags): (&str, &[&str]) = if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            ("wl-copy", &["--type", "text/html"])
        } else {
            ("xclip", &["-selection", "clipboard", "-t", "text/html"])
        };
        let mut child = Command::new(program)
            .args(flags)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .map_err(|e| textscribe::error::Error::Clipboard(format!("install {program}: {e}")))?;
        let copied = match child.stdin.take() {
            Some(mut stdin) => io::copy(&mut artifact, &mut stdin).map(|_| ()),
            None => Err(io::Error::other("clipboard stdin unavailable")),
        };
        let status = child.wait()?;
        copied.map_err(|e| textscribe::error::Error::Clipboard(e.to_string()))?;
        if !status.success() {
            return Err(textscribe::error::Error::Clipboard(status.to_string()).into());
        }
    }
    if args.browser {
        artifact.seek(SeekFrom::Start(0))?;
        let mut file = tempfile::Builder::new()
            .prefix("textscribe-")
            .suffix(".html")
            .tempfile()?;
        io::copy(&mut artifact, &mut file)?;
        file.flush()?;
        let (_, path) = file.keep().context("preserving browser document")?;
        let status = Command::new("xdg-open")
            .arg(&path)
            .stdout(Stdio::null())
            .status()
            .context("install xdg-utils to use --browser")?;
        if !status.success() {
            bail!("xdg-open exited with {status}");
        }
        eprintln!(
            "textscribe: browser document retained at {}",
            path.display()
        );
    }
    Ok(())
}
fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error)
            if error.chain().any(|e| {
                e.downcast_ref::<io::Error>()
                    .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe)
            }) =>
        {
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("textscribe: {error:#}");
            ExitCode::FAILURE
        }
    }
}
