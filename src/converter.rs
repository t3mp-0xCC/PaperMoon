use anyhow::Context;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

fn markdown_to_html(md_path: &Path) -> anyhow::Result<String> {
    let md_content = fs::read_to_string(md_path)
        .context("Failed to read md_content")?;
    let options = Options::all();
    let parser = Parser::new_ext(&md_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    Ok(html_content)
}

#[cfg(test)]
mod converter_tests {

    use super::*;

    #[test]
    fn md2html() {
        use std::fs::File;
        use std::io::{BufWriter, Write};
        let md_path = Path::new("./test/test.md");
        let html_path = Path::new("/tmp/test.html");
        let html_content = markdown_to_html(md_path)
            .expect("Failed to convert test markdown");
        let html_file = File::create(html_path)
            .expect("Failed to create test html file");
        let mut out = BufWriter::new(html_file);
        out.write_all(html_content.as_bytes())
            .expect("Failed to write html content to path");
    }
}
