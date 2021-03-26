use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

#[derive(Debug)] // Allows custom data types to be printable
struct CustomError(String);

fn main() -> Result<(),CustomError> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.input)
    .map_err(|err| CustomError(format!("Error reading `{}`: {}", args.input.display(), err)))?;

    // TODO: Do this with threads
    for line in content.lines() {
        let result = sudoku::parse_and_solve(line);
        let result_str: String = sudoku::puzzle_to_string(&result);
        sudoku::print_to_output(&result_str, &mut std::io::stdout());
    }
    Ok(())
}
