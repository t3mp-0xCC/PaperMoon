use anyhow::{anyhow, Context};
use easy_scraper::Pattern;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

use crate::cruds;

fn markdown_to_html(md_path: &Path) -> anyhow::Result<String> {
    let md_content = fs::read_to_string(md_path)
        .context("Failed to read md_content")?;
    let options = Options::all();
    let parser = Parser::new_ext(&md_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    Ok(html_content)
}

fn get_title_from_html(html_content: String) -> anyhow::Result<String> {
    let pat = Pattern::new(r#"
        <h1>{{title}}</h1>
    "#).unwrap();
    let matches = pat.matches(&html_content);
    // if content doesn't have title (<h1> tag)
    if matches.len() == 0 {
        return Ok("Untitled".to_string());
    }
    Ok(matches[0]["title"].clone())
}

pub fn importer(md_path: &Path) -> anyhow::Result<()> {
    let content_id = match md_path.file_stem() {
        Some(osstr) => match osstr.to_owned().into_string() {
            Ok(s) => s,
            Err(_) => return Err(anyhow!("Failed to convert OsStr to String")),
        },
        None => return Err(anyhow!("Failed to get conent_id")),
    };
    cruds::check_duplicate(&content_id)?;
    let html = markdown_to_html(md_path)?;
    let title = get_title_from_html(html.clone())?;
    cruds::create_post(&content_id, &title, &html)?;

    Ok(())
}

pub fn updater(md_path: &Path) -> anyhow::Result<()> {
    let content_id = match md_path.file_stem() {
        Some(osstr) => match osstr.to_owned().into_string() {
            Ok(s) => s,
            Err(_) => return Err(anyhow!("Failed to convert OsStr to String")),
        },
        None => return Err(anyhow!("Failed to get conent_id")),
    };
    let html = markdown_to_html(md_path)?;
    let title = get_title_from_html(html.clone())?;
    cruds::update_post(&content_id, &title, &html)?;
    Ok(())
}

pub fn deleter(md_path: &Path) -> anyhow::Result<()> {
    let content_id = match md_path.file_stem() {
        Some(osstr) => match osstr.to_owned().into_string() {
            Ok(s) => s,
            Err(_) => return Err(anyhow!("Failed to convert OsStr to String")),
        },
        None => return Err(anyhow!("Failed to get conent_id")),
    };
    cruds::delete_post(&content_id)?;
    Ok(())
}

#[cfg(test)]
mod article_tests {

    use super::*;

    #[test]
    #[ignore]
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
    #[ignore]
    fn get_title() {
        let html_path = Path::new("./test/test.html");
        let html_content = fs::read_to_string(html_path)
            .expect("Failed to read html path");
        let title = get_title_from_html(html_content)
            .expect("missing title");
        assert_eq!("title", title)
    }

    #[test]
    #[ignore]
    fn import_post() {
        let md_path = Path::new("./test/test.md");
        importer(md_path)
            .expect("Failed to import test Markdown");
    }

    #[test]
    #[ignore]
    fn updater_test() {
        let md_path = Path::new("./test/test.md");
        let new_md_path = Path::new("/tmp/test.md");
        importer(md_path)
            .expect("Failed to import test Markdown");
        std::process::Command::new("cp")
            .arg("test/test_another.md")
            .arg(new_md_path)
            .output()
            .expect("Failed to copy test md");
        updater(new_md_path)
            .expect("Failed to update test Markdown")
    }

    #[test]
    #[ignore]
    fn deleter_test() {
        let md_path = Path::new("./test/test.md");
        importer(md_path)
            .expect("Failed to import test Markdown");
        deleter(md_path)
            .expect("Failed to delete test Markdown");
    }
}
