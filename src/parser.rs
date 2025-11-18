use pulldown_cmark::Parser;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub fn collect_mk(path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut paths = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();

        if entry.file_type()?.is_dir() {
            let sub = collect_mk(&p)?;
            paths.extend(sub);
        } else {
            if let Some(ext) = p.extension() {
                if ext == "md" {
                    paths.push(p);
                }
            }
        }
    }

    Ok(paths)
}

pub fn parse_files(file_path: &PathBuf) -> Result<Vec<String>, Error> {
    let paths = collect_mk(&file_path)?;
    let mut parsed_files = Vec::new();
    for path in paths {
        let file = fs::read_to_string(path)?;

        parsed_files.push(file);
    }

    Ok(parsed_files)
}
