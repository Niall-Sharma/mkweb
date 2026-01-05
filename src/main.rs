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
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long, default_value = "./public/notes")]
        output: PathBuf,
    },
    Serve {
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        #[arg(long, default_value = "3000")]
        port: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build { input, output }) => {
            let parsed = match parse_files(&input) {
                Ok(p) => p,
                Err(e) => {
                    println!("Parser error: {}", e);
                    return;
                }
            };
            let generated_html = build::generate_html(parsed, &output);
            match generated_html {
                Ok(_result) => println!("Successfully generated html"),
                Err(e) => println!("Build Error {}", e),
            }
        }
        Some(Commands::Serve { host, port }) => match run(&host, &port) {
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
