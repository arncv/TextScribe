# TextScribe

Convert UTF-8 Markdown to HTML, EPUB, or PDF on stable Rust.

```sh
cargo build --release
textscribe doc.md > doc.html
textscribe < doc.md | gzip > doc.html.gz
textscribe doc.md --theme dark --embed-resources -o doc.html
textscribe doc.md --format epub -o doc.epub
textscribe doc.md --format pdf --pdf-engine /usr/bin/chromium -o doc.pdf
```

Input defaults to stdin; `-` also selects stdin. Output defaults to stdout,
except with `--clipboard` or `--browser`; `-o` can be combined with either.
Diagnostics and `--verbose` byte counts go to stderr. Broken pipes exit cleanly.

HTML defaults to a complete responsive document with an automatic light/dark
theme. Use `--fragment` for just the body. Fenced code gets escaped text and
language classes; no JavaScript or syntax-highlighting runtime is included.
Tables, strikethrough, task lists, and footnotes default on; disable them with
`--no-tables`, `--no-strikethrough`, `--no-tasklists`, or `--no-footnotes`.

`--embed-resources` (alias `--standalone`) embeds local Markdown images as data
URLs. Paths resolve against the input's parent, or the working directory for
stdin. `--resource-root` overrides that root. Canonical paths, including symlink
targets, must remain within the root. Remote URLs and percent-encoded paths
are rejected when embedding. Ordinary HTML output leaves image URLs unchanged.
Raw Markdown HTML is trusted and passed through; this is not an HTML sanitizer.
Only Markdown image events are embedded, not raw HTML/CSS references.

EPUB packages local images as archive resources regardless of the HTML embedding
flag, and escapes raw HTML to preserve XHTML validity. Remote images must first
be downloaded into the resource root. Archives stream into a temporary seekable
file before being copied to the selected destination; the archive is never held
in memory. Markdown and rendered HTML are held in memory because parsing and
image rewriting need document context. Temporary staging also prevents parse
errors from truncating the destination and supports binary stdout piping.

PDF needs an explicitly configured Chromium-compatible executable. It embeds
local images and uses an isolated temporary directory. Chromium must have a
working sandbox in the execution environment. Engine diagnostics use stderr.

Clipboard uses `wl-copy` on Wayland or `xclip` on X11. Browser mode uses
`xdg-open` and retains a temporary HTML file so asynchronous browsers can read it;
its path is printed to stderr for later removal. These optional integrations
currently target Linux desktops. Paths are passed as process arguments, never
interpreted as shell commands.

Dependencies: clap for CLI parsing, pulldown-cmark for Markdown, thiserror and
anyhow for errors, base64 for inline images, zip (stored entries, no compression
backend) for EPUB, and tempfile for safe staging.

```sh
cargo test
cargo check
cargo clippy --all-targets -- -D warnings
```
