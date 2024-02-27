use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use indicatif::ProgressBar;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let pb = ProgressBar::new(100);
    for _i in 0..100 {
        // do_hard_work();
        thread::sleep(Duration::from_millis(5));
        // pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");


    let stdout = io::stdout(); // get the global stdout entity
    // let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 42)?; // add `?` if you care about errors here

    

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display() ))?;


    rust_demo_rgrep::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

