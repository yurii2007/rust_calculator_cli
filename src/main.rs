use std::io::{ self, BufReader, Write, prelude::* };
use std::fs::File;
use anyhow::{ Result, Context };

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let pb = indicatif::ProgressBar::new(100);
    let args = Cli::parse();

    let file = File::open(&args.path).with_context(||
        format!("could not read a file {:?}", &args.path)
    )?;

    pb.inc(10);

    let reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    pb.inc(10);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(&args.pattern) {
                writeln!(handle, "{}", line)?;
            }
        }
        pb.inc(1);
    }

    pb.finish();
    Ok(())
}
