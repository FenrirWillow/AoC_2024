use clap::Parser;
use std::{collections::HashMap, env, path};

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

    // #region First part of the problem

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
        .iter()
        .zip(&c2)
        .map(|(n1, n2)| i32::abs(n1 - n2))
        .sum::<i32>();

    println!("Total distance is: {}", total_distance);

    // #endregion

    // #region Second part of the problem

    let mut occurrences = HashMap::<i32, i32>::new();

    c2.into_iter()
        .for_each(|number| *occurrences.entry(number).or_insert(0) += 1);

    let similarity_score = c1.iter().fold(0, |acc, number| {
        // TODO: (Stefan) I am not sure exactly what `or_insert` and `or_default` differ in. I am hoping,
        // that I am only probing the map and not trying to insert into it (unlike the one above). Also
        // don't like that "Default" is implicit from the trait `Default` for i32 - what if I want a weird
        // default?
        acc + number * (*occurrences.entry(*number).or_default())
    });

    println!("The similarity score is: {}", similarity_score);

    // #endregion
}
