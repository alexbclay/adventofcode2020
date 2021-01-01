use advent2020;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "d", long = "day", default_value = "1")]
    day: String,
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() {
    let args = Opt::from_args();

    // load the file
    let content = match std::fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Unable to read input file: {}", &args.input.display());
            eprintln!("Error was: {}", error);
            process::exit(1);
        }
    };
    // TODO: handle errors better
    let day = match &args.day[..] {
        "1" => advent2020::day_one::Solver::new(content),
        _ => {
            eprintln!("Day {} is not implemented yet", &args.day);
            process::exit(2);
        }
    };

    let day = match day {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Could not set up day {}: {:?}", &args.day, err);
            process::exit(4);
        }
    };

    match day.part_one() {
        Ok(solution) => println!("Solution Part 1: {}", solution),
        Err(err) => {
            println!("Error in part one: {}", err)
        }
    }
    match day.part_two() {
        Ok(solution) => println!("Solution Part 1: {}", solution),
        Err(err) => {
            println!("Error in part one: {}", err)
        }
    }
}
