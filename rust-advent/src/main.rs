use advent2020;
use advent2020::Solver;
use std::error::Error;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "d", long = "day", default_value = "1")]
    day: String,
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();

    // load the file
    let content = std::fs::read_to_string(&args.input)?;

    let day: Box<dyn Solver> = match &args.day[..] {
        "1" => advent2020::day_one::DayOneSolver::from_input(&content)?,
        "2" => advent2020::day_two::DayTwoSolver::from_input(&content)?,
        _ => {
            eprintln!("Day {} is not implemented yet", &args.day);
            process::exit(1);
        }
    };

    println!("Part 1: {}", day.part_one()?);
    println!("Part 2: {}", day.part_two()?);

    Ok(())
}
