use structopt::StructOpt;
use std::time::{Duration, Instant};


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
    // total time in miliseconds
    let mut total_time: u128 = 0;
    let mut count:u32 = 0;
    for line in content.lines() {
        let now = Instant::now();
        let result = sudoku::parse_and_solve(line);
        let elapsed = now.elapsed().as_millis();

        let elapsed_str: String = format!("miliseconds to solve puzzle = {}\n", elapsed);
        total_time += elapsed;
        count += 1;
        let result_str: String = sudoku::puzzle_to_string(&result);
        sudoku::print_to_output(&result_str, &mut std::io::stdout());
        sudoku::print_to_output(&elapsed_str, &mut std::io::stdout());
    }
    let average = total_time/(count as u128);
    let average_str: String = format!("{} runs took an average of {} ms", count, average);
    sudoku::print_to_output(&average_str, &mut std::io::stdout());
    Ok(())
}
