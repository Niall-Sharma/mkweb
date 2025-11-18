use pulldown_cmark::Parser;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::parser::ParsedFile;

pub fn generate_html(files: Vec<ParsedFile>, path_out: &PathBuf) -> Result<(), Error> {
    let file_extension = ".html";

    for file in files {
        let mut html_output = String::new();
        let content = file.content;
        let parser = Parser::new(&content);

        let stem = file.path.file_stem().unwrap().to_string_lossy();
        let new_file_path = format!("{}/{}{}", path_out.to_string_lossy(), stem, file_extension);

        pulldown_cmark::html::push_html(&mut html_output, parser);

        let mut file = File::create(new_file_path)?;
        file.write_all(html_output.as_bytes())?;
    }

    Ok(())
}
