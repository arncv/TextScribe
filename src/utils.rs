// src/utils.rs
use std::fs;
use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use clipboard::{ClipboardProvider, ClipboardContext};

pub fn get_theme_css(theme: &str) -> &str {
    match theme {
        "dark" => "<style>body { background-color: black; color: white; }</style>",
        "light" => "<style>body { background-color: white; color: black; }</style>",
        "solarized_dark" => "<style>body { background-color: #002b36; color: #839496; }</style>",
        "solarized_light" => "<style>body { background-color: #fdf6e3; color: #657b83; }</style>",
        "gruvbox_dark" => "<style>body { background-color: #282828; color: #ebdbb2; }</style>",
        "gruvbox_light" => "<style>body { background-color: #fbf1c7; color: #3c3836; }</style>",
        "dracula" => "<style>body { background-color: #282a36; color: #f8f8f2; }</style>",
        "monokai" => "<style>body { background-color: #272822; color: #f8f8f2; }</style>",
        "nord" => "<style>body { background-color: #2e3440; color: #d8dee9; }</style>",
        "zenburn" => "<style>body { background-color: #3f3f3f; color: #dcdccc; }</style>",
        "pookie" => "<style>body { background-color: pink; color: white; }</style>",
        _ => "", // default theme
    }
}

pub fn save_html_to_file(html: &str, output_file_path: &str) -> Result<(), std::io::Error> {
    fs::write(output_file_path, html)
}

pub fn copy_to_clipboard(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(html.to_string())?;
    Ok(())
}

pub fn export_html_to_pdf(html: &str, output_pdf_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("wkhtmltopdf")
        .arg("-")
        .arg(output_pdf_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child.stdin.take().expect("piped stdin").write_all(html.as_bytes())?;
    let output = child.wait_with_output()?;
    
    if output.status.success() {
        println!("Successfully exported to PDF: {}", output_pdf_path);
    } else {
        return Err(format!("PDF export failed ({}): {}", output.status, String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}
