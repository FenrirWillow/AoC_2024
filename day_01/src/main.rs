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

    let input_data = aocutils::read_input_file_by_line(cli_inputpath, crate_name);

    let (first_column, second_column) = input_data
        .iter()
        .flat_map(|line| line.split_whitespace().map(|c| c.parse::<i32>().unwrap()))
        .enumerate()
        .partition::<Vec<_>, _>(|&(i, _)| i % 2 == 0);

    let mut c1 = first_column
        .into_iter()
        .map(|(_idx, value)| value)
        .collect::<Vec<i32>>();

    let mut c2 = second_column
        .into_iter()
        .map(|(_idx, value)| value)
        .collect::<Vec<i32>>();

    c1.sort_unstable();
    c2.sort_unstable();

    let total_distance = c1
        .into_iter()
        .zip(c2)
        .map(|(n1, n2)| i32::abs(n1 - n2))
        .sum::<i32>();

    println!("Total distance is: {}", total_distance);
}
