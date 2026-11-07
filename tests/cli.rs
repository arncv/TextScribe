use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("textscribe-test-{}-{}", std::process::id(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir(&path).unwrap();
    path
}

#[test]
fn missing_input_returns_failure() {
    let dir = temp_dir();
    let status = Command::new(env!("CARGO_BIN_EXE_textscribe"))
        .current_dir(&dir).args(["missing.md", "output.html"]).status().unwrap();
    assert!(!status.success());
    fs::remove_dir_all(dir).unwrap();
}

#[cfg(unix)]
#[test]
fn pdf_receives_converted_html_and_requested_filename() {
    use std::os::unix::fs::PermissionsExt;
    let dir = temp_dir();
    fs::write(dir.join("input.md"), "# PDF content\n").unwrap();
    let executable = dir.join("wkhtmltopdf");
    fs::write(&executable, "#!/bin/sh\n[ \"$1\" = - ] || exit 2\n/bin/cat > \"$2\"\n").unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_textscribe"))
        .current_dir(&dir).env("PATH", &dir)
        .args(["input.md", "--pdf", "--pdf-name", "custom.pdf"]).status().unwrap();
    assert!(status.success());
    assert!(fs::read_to_string(dir.join("custom.pdf")).unwrap().contains("<h1>PDF content</h1>"));
    fs::remove_dir_all(dir).unwrap();
}
