mod build;
mod parser;
mod server;

use clap::{Parser, Subcommand};
use parser::parse_files;
use server::run;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "mkweb",
    version,
    about = "Convert an Obsidian vault into a static website"
)]
pub struct Cli {
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output directory for the generated site
    #[arg(short, long, default_value = "./public/notes")]
    pub output: PathBuf,

    /// Optional: clean output directory before building
    #[arg(long)]
    pub clean: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Build,
    Serve,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build) => {
            let parsed = match parse_files(&cli.input) {
                Ok(p) => p,
                Err(e) => {
                    println!("Parser error: {}", e);
                    return;
                }
            };
            let generated_html = build::generate_html(parsed, &cli.output);
            match generated_html {
                Ok(_result) => println!("Successfully generated html"),
                Err(e) => println!("Build Error {}", e),
            }
        }
        Some(Commands::Serve) => match run() {
            Ok(p) => p,
            Err(e) => {
                println!("Error starting server: {}", e);
                return;
            }
        },
        None => {
            println!("No command provided. Try --help.");
        }
    }
}
