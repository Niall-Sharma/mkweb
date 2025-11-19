use crate::parser::ParsedFile;
use pulldown_cmark::Parser;
use std::fs::{self, File};
use std::io::{Error, Read, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct HtmlFile {
    pub content: String,
}

pub fn generate_html(
    parsed_files: Vec<ParsedFile>,
    path_in: &PathBuf,
    path_out: &PathBuf,
) -> Result<(), Error> {
    let file_extension = "html";

    let mut base_out = PathBuf::from(path_out);
    base_out.push(path_in.file_name().unwrap());

    fs::create_dir_all(&base_out)?;

    for file in parsed_files {
        let mut html_output = String::new();
        let parser = Parser::new(&file.content);
        pulldown_cmark::html::push_html(&mut html_output, parser);

        let updated_file = inject_css(HtmlFile {
            content: html_output,
        })?;

        let mut out_path = base_out.clone();

        let stem = file.path.file_stem().unwrap().to_string_lossy().to_string();

        out_path.push(format!("{}.{}", stem, file_extension));

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut out_file = File::create(&out_path)?;
        out_file.write_all(updated_file.content.as_bytes())?;
    }

    Ok(())
}

pub fn inject_css(mut generated_file: HtmlFile) -> Result<HtmlFile, Error> {
    let html_layout = File::open("./src/layout.html");
    let mut layout_content = String::new();

    html_layout?.read_to_string(&mut layout_content)?;
    let merged_content = layout_content.replace("{{content}}", &generated_file.content);
    generated_file.content = merged_content;
    Ok(generated_file)
}
