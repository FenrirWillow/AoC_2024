use std::{
    env, fs,
    io::{self, BufRead},
    path,
};

// TODO: (Stefan) Figure out how logging should work!

/// TODO: (Stefan) Document me!
pub fn adjust_input_filepath(
    cli_provided_path: &path::Path,
    executing_crate: &str,
) -> path::PathBuf {
    if let Ok(root) = env::var("BUILD_WORKING_DIRECTORY") {
        let full_path = path::Path::new(&root)
            .join(executing_crate)
            .join("inputs")
            .join(cli_provided_path);

        return full_path;
    }

    cli_provided_path.to_path_buf()
}

/// TODO: (Stefan) Document me!
pub fn read_input_file_whole(cli_provided_path: &path::Path, executing_crate: &str) -> String {
    let input_filepath = adjust_input_filepath(cli_provided_path, executing_crate);

    fs::read_to_string(input_filepath).unwrap()
}

/// TODO: (Stefan) Document me!
pub fn read_input_file_by_line(
    cli_provided_path: &path::Path,
    executing_crate: &str,
) -> Vec<String> {
    let input_filepath = adjust_input_filepath(cli_provided_path, executing_crate);

    let file = fs::File::open(input_filepath).unwrap();

    io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
}
