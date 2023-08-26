# Markdown to HTML Converter

The Markdown to HTML Converter is a command-line tool written in Rust that allows you to convert Markdown files to HTML. It utilizes the `pulldown-cmark` crate to parse Markdown and generate corresponding HTML output.

## Features

- Convert Markdown files to HTML.
- Enhanced command-line interface with argument parsing.
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
cargo run --release -- -i <input-file> -o <output-file> [--theme <theme-name>]


```



Replace <input-file> with the path to your Markdown file and <output-file> with the desired path for the HTML output file. Optionally, specify a theme with the --theme flag (available themes: default, dark, light).

For example, to convert a file named example.md to HTML with a dark theme and save it as output.html, run the following command:

```console
cargo run --release -- -i example.md -o output.html --theme dark


```



The HTML output will be saved to the specified file.

## Options

-Theming: Use the --theme flag followed by a theme name to specify the theme for the output HTML. Available themes are default, dark, and light.
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