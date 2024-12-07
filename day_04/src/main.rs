use clap::Parser;
use std::{env, path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "example.txt")]
    input: String,
}

fn main() {
    let args = Args::parse();
    let crate_name = clap::crate_name!();
    let cli_inputpath = path::Path::new(&args.input);

    println!("Hello to solver for {crate_name}");

    // TODO: (Stefan) Solve today's problem! ⭐⭐
}
