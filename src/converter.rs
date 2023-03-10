use anyhow::Context;use easy_scraper::Pattern;
use log::debug;
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

fn article_converter(md_path: &Path) -> anyhow::Result<()> {
    Ok(())
}

fn get_title_from_html(html_path: &Path) -> anyhow::Result<String> {
    let pat = Pattern::new(r#"
        <h1>{{title}}</h1>
    "#).unwrap();
    let html_content = fs::read_to_string(html_path)
        .context("Failed to load html content")?;
    let matches = pat.matches(&html_content);
    // if content doesn't have title (<h1> tag)
    if matches.len() == 0 {
        return Ok("Untitled".to_string());
    }
    Ok(matches[0]["title"].clone())
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

    #[test]
    fn get_title() {
        let html_path = Path::new("./test/test.html");
        let title = get_title_from_html(html_path)
            .expect("missing title");
        assert_eq!("title", title)
    }
}
