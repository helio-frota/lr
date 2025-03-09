use std::{fs, io, path::Path};

mod util;
use clap::Parser;

#[cfg(test)]
mod test;

#[derive(Parser, Debug)]
#[command(name = "dpc")]
#[command(bin_name = "dpc")]
struct Cli {
    /// Rust code root directory
    #[arg(short, value_name = "DIR", default_value_t = String::from("."))]
    d: String,
    /// 1.0 for exact same code blocks (Using 0.98 for similar blocks)
    #[arg(short, value_name = "threshold", default_value_t = 0.98)]
    t: f64,
    /// Ignores scanning with the provided dir name
    #[arg(short, value_name = "ignore", default_value_t = String::from("duplicrabs"))]
    i: String,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();

    //let mut threshold = args.t;
    let dir = args.d;
    let last_dir = Path::new(&dir)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("Something happened...");

    let rust_files = util::rust_files(&dir, args.i.as_str())?;

    let mut filtered_code_blocks: Vec<String> = Vec::new();

    for r in &rust_files {
        // Some stuff to show the file name on duplicrabs report later.
        let r_as_string = r.to_string_lossy();
        let the_index = r_as_string
            .find(last_dir)
            .unwrap_or_else(|| r_as_string.len());
        let file_name = &r_as_string[the_index..];

        let content = fs::read_to_string(r.to_string_lossy().into_owned().as_str())?;
        let cbs = util::code_blocks(content.as_str());

        for cb in &cbs {
            let f = util::block_counter(cb);
            // at least 150+ chars and 2+ lines...
            if f.0 >= 150 && f.1 >= 2 {
                let b = format!("{}{}{}", cb, "\n", file_name);
                filtered_code_blocks.push(b);
            }
        }
    }

    let similar_blocks = util::similar(filtered_code_blocks, args.t);
    util::report(similar_blocks);

    Ok(())
}
