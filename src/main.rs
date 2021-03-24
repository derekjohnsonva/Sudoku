use structopt::StructOpt;
use std::io::Write;


#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,
}

#[derive(Debug)] // Allows custom data types to be printable
struct CustomError(String);

fn main() -> Result<(),CustomError> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.input)
    .map_err(|err| CustomError(format!("Error reading `{}`: {}", args.input.display(), err)))?;

    let mut output_file = std::fs::File::create(&args.output)
    .map_err(|err| CustomError(format!("Error reading `{}`: {}", args.output.display(), err)))?;
    // TODO: Do this with threads
    for line in content.lines() {
        let result = sudoku::parse_and_solve(line);
        let result_str: String = sudoku::puzzle_to_string(&result);
        output_file.write_all(result_str.as_bytes()).expect("write failed");
    }
    Ok(())
}
