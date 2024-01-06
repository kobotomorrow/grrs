use clap::Parser;
use anyhow::{Context, Result};
use std::{path, fs::{self}, io::{BufReader, BufRead, BufWriter, stdout}};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: path::PathBuf
}

fn main() -> Result<()> {
    let args = Cli::parse();
    // ファイルの内容を一度にメモリに展開する
    // let content = fs::read_to_string(&args.path).expect("could not read file");
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    // ファイルの内容を行ごとにメモリに展開する
    let file = fs::File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let lines = BufReader::new(file).lines();
    let lines = lines.map(|line| line.unwrap()).collect::<Vec<String>>();
    let writer = BufWriter::new(stdout());
    grrs::find_matches(writer, lines, &args.pattern)?;
    Ok(())
}
