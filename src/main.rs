use clap::Parser;
use anyhow::{Context, Result};
use std::{path, fs::{self}, io::{BufReader, BufRead, BufWriter, stdout, Write}};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: path::PathBuf
}

fn find_matches(mut writer: impl Write, lines: Vec<String>, pattern: &str) -> Result<()> {
    for line in lines {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result_line = Vec::new();
    let mut lines = Vec::new();

    lines.push(String::from("hogefuga"));
    lines.push(String::from("foobar"));

    let result = find_matches(&mut result_line, lines, "fuga");
    assert!(result.is_ok());
    assert_eq!(result_line, b"hogefuga\n");
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
    let file = fs::File::open(&args.path).with_context(|| format!("could not readfile `{}`", args.path.display()))?;
    let lines = BufReader::new(file).lines();
    let lines = lines.map(|line| line.unwrap()).collect::<Vec<String>>();
    let writer = BufWriter::new(stdout());
    find_matches(writer, lines, &args.pattern)?;
    Ok(())
}
