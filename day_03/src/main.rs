use clap::Parser;
use regex;
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

    let input_data = aocutils::read_input_file_whole(cli_inputpath, crate_name);

    let re = regex::Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    let total_mul_sum = re
        .captures_iter(&input_data)
        .map(|capture_group| capture_group.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum::<i32>();

    println!("Sum of all found: {total_mul_sum}",);
}
