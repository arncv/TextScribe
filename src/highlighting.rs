// src/highlighting.rs
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;

pub fn apply_syntax_highlighting(html: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let theme = &theme_set.themes["base16-ocean.dark"];
    let mut highlighted_html = String::new();

    for line in html.lines() {
        if line.starts_with("<pre><code>") && line.ends_with("</code></pre>") {
            let code = &line[11..line.len() - 12];
            let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
            let highlighted = highlighted_html_for_string(code, &syntax_set, syntax, theme);
            highlighted_html.push_str(&format!("<pre><code>{}</code></pre>", highlighted));
        } else {
            highlighted_html.push_str(line);
        }
    }

    highlighted_html
}
