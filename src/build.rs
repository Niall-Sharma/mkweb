use pulldown_cmark::Parser;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::PathBuf;

use crate::parser::ParsedFile;

#[derive(Debug)]
pub struct HtmlFile {
    pub content: String,
}

pub fn generate_html(parsed_files: Vec<ParsedFile>, path_out: &PathBuf) -> Result<(), Error> {
    let file_extension = ".html";

    for file in parsed_files {
        let mut html_output = String::new();
        let content = file.content;
        let parser = Parser::new(&content);

        let stem = file.path.file_stem().unwrap().to_string_lossy();
        let new_file_path = format!("{}/{}{}", path_out.to_string_lossy(), stem, file_extension);

        pulldown_cmark::html::push_html(&mut html_output, parser);
        let updated_file = inject_css(HtmlFile {
            content: html_output,
        });
        let mut file = File::create(new_file_path)?;
        file.write_all(updated_file?.content.as_bytes())?;
    }

    Ok(())
}

pub fn inject_css(mut generated_file: HtmlFile) -> Result<HtmlFile, Error> {
    let html_layout = File::open("./public/layout.html");
    let mut layout_content = String::new();

    html_layout?.read_to_string(&mut layout_content)?;
    let merged_content = layout_content.replace("{{content}}", &generated_file.content);
    generated_file.content = merged_content;
    Ok(generated_file)
}
