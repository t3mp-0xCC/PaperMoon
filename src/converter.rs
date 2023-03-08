use anyhow::{Context, Result};
use pulldown_cmark::{html, Options, Parser};
use std::fs;

fn markdown_to_html(md_path: &str) -> Result<String> {
    let md_content = fs::read_to_string(md_path)
        .context("Failed to read md_content")?;
    let options = Options::all();
    let parser = Parser::new_ext(&md_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    Ok(html_content)
}
