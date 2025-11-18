use pulldown_cmark::Parser;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub fn generate_html(contents: Vec<String>) -> Result<String, Error> {
    let mut html_output = String::new();
    for content in contents {
        let parser = Parser::new(&content);
        pulldown_cmark::html::push_html(&mut html_output, parser);
    }

    Ok(html_output)
}
