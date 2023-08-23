# Markdown to HTML Converter

The Markdown to HTML Converter is a command-line tool written in Rust that allows you to convert Markdown files to HTML. It utilizes the `pulldown-cmark` crate to parse Markdown and generate corresponding HTML output.

## Features

- Convert Markdown files to HTML.
- Support for common Markdown syntax, including headings, paragraphs, lists, emphasis, links, images, and code blocks.
- Options to enable tables and strikethrough formatting.
- Save the generated HTML to a file.

## Installation

1. Ensure you have Rust and Cargo installed. If not, follow the installation instructions at [Rust's official website](https://www.rust-lang.org/).

2. Clone this repository:

```console
git clone https://github.com/arncv/MDtoHTM.git

```


3. Navigate to the project directory:

```console
**cd markdown-to-html-converter**

```


1. Build the project using Cargo:

```console
cargo build --release

```



## Usage

To convert a Markdown file to HTML, run the following command:

```console
cargo run -- <input-file> <output-file>

```



Replace `<input-file>` with the path to your Markdown file and `<output-file>` with the desired path for the HTML output file.

For example, to convert a file named `example.md` to HTML and save it as `output.html`, run the following command:

```console
cargo run -- example.md output.html

```



The HTML output will be saved to the specified file.

## Options

- Enable Tables: To enable support for tables in the Markdown input, pass the `--tables` flag:

```console
cargo run -- <input-file> <output-file> --tables

```


- Enable Strikethrough: To enable support for strikethrough formatting in the Markdown input, pass the `--strikethrough` flag:

```console
cargo run -- <input-file> <output-file> --strikethrough

```



## Contributing

Contributions are welcome! If you have any ideas, improvements, or bug fixes, please submit a pull request or open an issue.

## License

This project is licensed under the [MIT License](LICENSE).