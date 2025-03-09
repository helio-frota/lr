use std::io;
use std::path::PathBuf;

use strsim::sorensen_dice;
use walkdir::WalkDir;

/// This function gets all the .rs files from a directory, and returns a
/// vector with all the paths of the .rs files.
pub fn rust_files(dir: &str, ignore: &str) -> Result<Vec<PathBuf>, io::Error> {
    let ignore_list: Vec<&str> = ignore.split(',').collect();
    let files = WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            let path_str = path.to_string_lossy();
            !path_str.contains("target")
                && !ignore_list.iter().any(|&ignore| path_str.contains(ignore))
        })
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("rs"))
        .map(|entry| entry.path().to_path_buf())
        .collect();

    Ok(files)
}

pub fn code_blocks(content: &str) -> Vec<String> {
    // Changes the content to a vector lines.
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut i = 0;
    let mut code_blocks: Vec<String> = Vec::new();
    // loop the lines and discard some stuff.
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.is_empty()
            || trimmed.contains("use")
            || trimmed.starts_with("use")
            || trimmed.contains("use crate")
            || trimmed.contains("pub mod")
            || trimmed.starts_with("//")
            || trimmed.starts_with("///")
        {
            i += 1;
            continue;
        }

        // Creates a temporary code-block vector of lines.
        let mut block_lines: Vec<String> = Vec::new();
        block_lines.push(lines[i].clone());

        let mut j = i + 1;
        while j < lines.len() && !lines[j].trim().is_empty() {
            block_lines.push(lines[j].clone());
            j += 1;
        }

        if block_lines.len() > 1 {
            code_blocks.push(block_lines.join("\n"));
        }

        i = j;
    }
    code_blocks
}

// This counts lines and chars from code-block
// to make opinionated checks later.
pub fn block_counter(cb: &str) -> (u16, u8) {
    let mut chars = 0;
    let mut lines = 0;

    for line in cb.lines() {
        let trimmed = line.trim();
        chars += trimmed.len() as u16;
        lines += 1;
    }

    (chars, lines)
}

pub fn similar(blocks: Vec<String>, threshold: f64) -> Vec<(String, String, f64)> {
    let mut result = Vec::new();

    for i in 0..blocks.len() {
        for j in i + 1..blocks.len() {
            let b1 = blocks[i].as_str();
            let b2 = blocks[j].as_str();
            let similarity = sorensen_dice(b1, b2);

            if similarity >= threshold {
                let t = (b1.to_string(), b2.to_string(), similarity);
                result.push(t);
            }
        }
    }

    result
}

pub fn report(similar_blocks: Vec<(String, String, f64)>) -> String {
    let mut out = String::new();
    if !similar_blocks.is_empty() {
        out.push_str("# Duplicrabs\n\n");

        let mut exact = String::new();
        let mut almost = String::new();

        let mut exact_idx = 1;
        let mut almost_idx = 1;

        for s in similar_blocks.iter() {
            let mut b1: Vec<&str> = s.0.split('\n').collect();
            let f1 = b1.pop();
            let mut b2: Vec<&str> = s.1.split('\n').collect();
            let f2 = b2.pop();

            let mut block = String::new();

            if s.2 == 1.0 {
                block.push_str(format!("### \u{1F980} {}\n\n", exact_idx).as_str());
                exact_idx += 1;
            } else {
                block.push_str(format!("### \u{1F980} {}\n\n", almost_idx).as_str());
                almost_idx += 1;
            }

            block.push_str("```rust\n");
            block.push_str(format!("{}\n", b1.join("\n").as_str()).as_str());
            block.push_str("```\n\n");

            block.push_str(format!("`{}`\n\n", f1.unwrap_or("n/a")).as_str());

            block.push_str("```rust\n");
            block.push_str(format!("{}\n", b2.join("\n").as_str()).as_str());
            block.push_str("```\n\n");

            block.push_str(format!("`{}`\n\n", f2.unwrap_or("n/a")).as_str());

            if s.2 == 1.0 {
                exact.push_str(&block);
            } else {
                almost.push_str(&block);
            }
        }

        if !exact.is_empty() {
            out.push_str("> [!TIP]\n> Exactly the same\n\n");
            out.push_str(&exact);
        }

        if !almost.is_empty() {
            out.push_str("> [!WARNING]\n> Almost the same\n\n");
            out.push_str(&almost);
        }
    }
    println!("{out}");
    out
}
