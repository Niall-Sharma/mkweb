mod build;
mod parser;

use clap::{Parser, Subcommand};
use parser::parse_files;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "mkweb",
    version,
    about = "Convert an Obsidian vault into a static website"
)]
pub struct Cli {
    /// Path to the Obsidian vault
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output directory for the generated site
    #[arg(short, long, default_value = "public")]
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
    Watch,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build) => {
            println!("Building site from {:?} into {:?}", cli.input, cli.output);

            let parsed = parse_files(&cli.input).unwrap();

            let generated_html = build::generate_html(parsed, &cli.input, &cli.output);
            match generated_html {
                Ok(_result) => println!("Successfully generated html"),
                Err(e) => println!("Error {}", e),
            }
        }
        Some(Commands::Watch) => {
            println!(
                "Watching {:?} and rebuilding into {:?}",
                cli.input, cli.output
            );
            // call your watch logic here
        }
        None => {
            println!("No command provided. Try --help.");
        }
    }
}
