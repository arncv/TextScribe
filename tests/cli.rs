use std::{
    io::Write,
    process::{Command, Stdio},
};
fn cli(args: &[&str], input: &[u8]) -> std::io::Result<std::process::Output> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_textscribe"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input)?;
    }
    child.wait_with_output()
}
#[test]
fn stdin_and_flags() -> Result<(), Box<dyn std::error::Error>> {
    for args in [vec!["--fragment"], vec!["-", "--fragment"]] {
        let result = cli(&args, b"~~gone~~\n")?;
        assert!(result.status.success());
        assert_eq!(
            String::from_utf8(result.stdout)?,
            "<p><del>gone</del></p>\n"
        );
        assert!(result.stderr.is_empty());
    }
    let result = cli(&["--fragment", "--no-strikethrough"], b"~~gone~~")?;
    assert!(String::from_utf8(result.stdout)?.contains("~~gone~~"));
    Ok(())
}
#[test]
fn errors() -> Result<(), Box<dyn std::error::Error>> {
    assert!(
        !cli(&["/definitely-missing-textscribe.md"], b"")?
            .status
            .success()
    );
    let result = cli(&["--format", "pdf"], b"hello")?;
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr)?.contains("--pdf-engine"));
    assert!(
        !cli(&["--format", "epub", "--fragment"], b"")?
            .status
            .success()
    );
    assert!(!cli(&[], &[255])?.status.success());
    Ok(())
}
#[test]
fn file_output_and_epub_pipe() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let input = dir.path().join("in.md");
    let output = dir.path().join("out.html");
    std::fs::write(&input, "# File")?;
    let result = Command::new(env!("CARGO_BIN_EXE_textscribe"))
        .arg(input)
        .arg("-o")
        .arg(&output)
        .output()?;
    assert!(result.status.success());
    assert!(result.stdout.is_empty());
    assert!(std::fs::read_to_string(output)?.contains("<h1>File</h1>"));
    let result = cli(&["--format", "epub"], b"# Book")?;
    assert!(result.status.success());
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(result.stdout))?;
    assert_eq!(archive.by_index(0)?.name(), "mimetype");
    assert!(archive.by_name("EPUB/content.xhtml").is_ok());
    Ok(())
}
