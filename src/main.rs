#![deny(unstable_features)]
#![deny(unused_features)]

use anyhow::Result;
use clap::Parser;
use pdf_extract::extract_text;
use std::{
    io::{self, Write},
    path::PathBuf,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

const APP_NAME: &str = "pdf2text";
const APP_SHORT_ABOUT: &str = "A tool to extract text from PDF files";
#[derive(Parser, Debug)]
#[command(name = APP_NAME)]
#[command(about = APP_SHORT_ABOUT, long_about = None)]
#[command(version, subcommand_required = false)]
struct Cli {
    /// Path to the PDF that should be processed
    #[arg(value_name = "PDF_PATH", value_hint = clap::ValueHint::FilePath)]
    pdf_path: PathBuf,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let text = extract_text(&cli.pdf_path)?;
    let mut out = io::BufWriter::new(io::stdout());
    out.write_all(text.as_bytes())?;
    Ok(())
}
