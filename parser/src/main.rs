use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use eval::Env;
use figlet_rs::FIGfont;
use std::path::PathBuf;

mod ast;
mod eval;
mod parse;

#[derive(Parser, Debug)]
#[command(name = "set_parser", version, about = "A tiny DSL for integer sets")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Parse { file: PathBuf },
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file } => parse(file),
        Commands::Credits => credits(),
    }
}

fn parse(file: PathBuf) -> Result<()> {
    let path = file
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {}", file.display()))?;

    let src = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read source: {}", path.display()))?;

    let program =
        parse::parse_program(&src).with_context(|| format!("Parse error in {}", path.display()))?;

    let mut env = Env::default();
    match env.eval_program(&program) {
        Ok(results) => {
            for s in results {
                println!("{}", Env::format_set(&s));
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {e}");
            Err(anyhow::anyhow!(e))
        }
    }
}

fn credits() -> Result<()> {
    let standard_font = FIGfont::standard().unwrap();
    let output = standard_font.convert("Set Parser");
    println!("{}", output.unwrap());

    println!(
        "Set Parser is a tiny DSL for integer set algebra, which lets you declare, compare and operate on sets."
    );
    println!("Repository and more info: https://github.com/katerynabratiuk/SetParser");
    println!("Use set_parser --help for more information.");
    Ok(())
}
