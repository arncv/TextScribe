TextScribe - Markdown to HTML Converter
==========================
![logo](https://i.imgur.com/FU0mh9C.png)

The Markdown  Converter is a robust command-line utility built with Rust. It's designed to transform Markdown files into HTML & PDF format effortlessly. With the power of the `pulldown-cmark` crate, it ensures accurate parsing and rendering of Markdown content.

🌟 Features
-----------

*   **Swift Conversion**: Instantly convert your Markdown files into HTML.
*   **Intuitive CLI**: A user-friendly command-line interface powered by the `clap` crate.
*   **Rich Markdown Support**: Supports a wide range of Markdown syntax, including headings, paragraphs, lists, emphasis, links, images (with Base64 embedding), and code blocks.
*   **Customization Options**: Choose specific Markdown features like tables and strikethrough using the `pulldown-cmark` Options.
*   **Flexible Output**: Save the generated HTML to a specified location, copy it directly to the clipboard, or preview it in your default web browser.
*   **Theming**: Style your HTML output with different themes.

🚀 Installation
---------------

1.  **Setup Rust Environment**: If you haven't installed Rust and Cargo, get them from [Rust's official website](https://www.rust-lang.org/).
2.  **Clone the Repository**:
    

    ```console
    git clone https://github.com/arncv/MDtoHTM.git
    ```
    
3.  **Navigate to the Project Directory**:
    

    
    ```console
    cd markdown-to-html-converter
    ```
    
4.  **Compile the Project**:
    

    
    ```console
    cargo build --release
    ```
    

🛠 Usage
--------

To convert your Markdown to HTML, use the following command:



```console
cargo run --release -- -i <input-file> [-o <output-file>] [--theme <theme-name>] [--clipboard] [--browser]
```

*   `<input-file>`: Path to your Markdown file.
*   `<output-file>`: (Optional) Path for the HTML output. If not provided and neither clipboard nor browser options are used, an error will be prompted.
*   `--theme <theme-name>`: Choose a theme (options: default, dark, light).
*   `--clipboard`: Copy the generated HTML directly to the clipboard.
*   `--browser`: Preview the generated HTML in your default web browser.

**Example**: Convert `example.md` to HTML using the dark theme and save it as `output.html`:



```console
cargo run --release -- -i example.md -o output.html --theme dark
```

To copy the output directly to the clipboard:



```console
cargo run --release -- -i example.md --clipboard
```

To preview the output in your default web browser:



```console
cargo run --release -- -i example.md --browser
```

🎨 Options
----------

*   **Theming**: Style your HTML output.
    

    
    ```console
    cargo run --release -- -i <input-file> -o <output-file> --theme <theme-name>
    ```
    
*   **Tables**: Enable table formatting in your Markdown.
    

    
    ```console
    cargo run --release -- -i <input-file> -o <output-file> --tables
    ```
    
*   **Strikethrough**: Enable strikethrough formatting.
    

    
    ```console
    cargo run --release -- -i <input-file> -o <output-file> --strikethrough
    ```
    
*   **Clipboard Output**: Copy the generated HTML to the clipboard.
    
    ```console
    cargo run --release -- -i <input-file> --clipboard
    ```
    
*   **Browser Preview**: View the generated HTML in your default web browser.

    
    ```console
    cargo run --release -- -i <input-file> --browser
    ```
    


🚧 Future Roadmap
-----------------

*   ✅**Support more image types**: Support for more image types ; GIFs, PNG, JPEG.
*   ✅**Improved Error Handling**:  Improved functionality & logging for errors.
*   ✅**PDF Conversion**: Integrate functionality to convert the generated HTML into PDF format.
*   **Extended Theming**: Introduce more themes and customization options for the HTML output.
*   **Interactive GUI**: Develop a graphical user interface for users who prefer GUI over CLI.
*   **Enhanced Image Support**: Improve image embedding with options for resizing, alignment, and captions.

We're always open to suggestions and feedback. If you have an idea that's not listed here, please share it with us!

🤝 Contributing
---------------

Contributions are always welcome! Whether it's a feature request, bug fix, or a new idea, feel free to submit a pull request or open an issue. Let's enhance this tool together!

📜 License
----------

This project is licensed under the [MIT License](LICENSE).
