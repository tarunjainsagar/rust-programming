#![allow(unused)]
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let input_file = File::open(&args.path)?;
    let reader = BufReader::new(input_file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
