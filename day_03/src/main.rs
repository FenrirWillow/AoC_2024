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

    let input_data = aocutils::read_input_file_whole(cli_inputpath, crate_name);

    // #region Part One

    let only_mul_ops_re = regex::Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    let total_mul_sum = only_mul_ops_re
        .captures_iter(&input_data)
        .map(|capture_group| capture_group.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum::<i32>();

    println!("Total sum of all mul ops: {total_mul_sum}",);

    // #endregion

    // #region Part Two

    let mul_ops_and_ctrl_re =
        regex::Regex::new(r"(?<ops>mul\((?<a>\d{1,3}),\s*(?<b>\d{1,3})\))|(?<ctrl>do(?:n't)?\(\))")
            .unwrap();

    let mut accept_new_ops = true;

    let total_mul_sum_with_ctrl = mul_ops_and_ctrl_re
        .captures_iter(&input_data)
        .map(|capture_group| {
            if capture_group.name("ops").is_some() {
                if accept_new_ops {
                    let a = capture_group
                        .name("a")
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .unwrap();
                    let b = capture_group
                        .name("b")
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .unwrap();

                    return a * b;
                } else {
                    return 0;
                }
            } else if let Some(control_op) = capture_group.name("ctrl") {
                accept_new_ops = control_op.as_str() == "do()";
                return 0;
            }

            unreachable!("The capture group was neither a mul op NOR a control op!");
        })
        .sum::<i32>();

    println!("Total sum of all mul ops (respecing control ops): {total_mul_sum_with_ctrl}",)

    // #endregion
}
