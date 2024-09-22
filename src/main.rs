use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use anyhow::{ Result, Context };

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path).with_context(|| format!("could not read a file {:?}", &args.path))?;

    // let file = match result {
    //     Ok(file) => { file }
    //     Err(error) => {
    //         return Err(
    //             format!("Error reading file {:?}: {}", &args.path.to_str(), error.to_string())
    //         );
    //     }
    // };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
