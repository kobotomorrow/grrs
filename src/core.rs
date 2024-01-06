use anyhow::Result;
use std::io::Write;

pub fn find_matches(mut writer: impl Write, lines: Vec<String>, pattern: &str) -> Result<()> {
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
