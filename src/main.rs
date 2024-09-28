use signal_hook::{ consts::SIGINT, iterator::Signals };
use crossbeam_channel::{ bounded, tick, Receiver, select };
use std::io::{ self, BufReader, Write, prelude::* };
use std::fs::File;
use std::thread;
use std::time::Duration;
use anyhow::{ Result, Context };
use serde::{ Serialize, Deserialize };

use clap::Parser;

#[derive(Debug, Serialize, Deserialize, Default)]
struct ConfigFile {
    name: String,
    comfy: bool,
    test: i64,
}

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let cfg: ConfigFile = confy::load("calculator_cli", "ConfigFile")?;
    println!("{:?}", cfg);
    let pb = indicatif::ProgressBar::new(100);
    let args = Cli::parse();

    // ctrlc::set_handler(|| {
    //     println!("Ctrl + C pressed");
    // }).expect("Error setting handler for Ctrl + C");

    let mut signals = Signals::new([SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("recevied signal {:?}", sig);
        }
    });

    let file = File::open(&args.path).with_context(||
        format!("could not read a file {:?}", &args.path)
    )?;

    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye");
                break;
            }
        }
    }

    pb.inc(10);

    thread::sleep(Duration::from_secs(2));
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

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded::<()>(100);
    let _ = ctrlc::set_handler(move || {
        let _ = sender.send(());
    });

    Ok(receiver)
}
