use advent2020;
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
    println!("Args: {:?}", args);

    // load the file
    // TODO: handle this error
    let content = match std::fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(error) => error.to_string(),
    };
    // println!("{}", content);
    // TODO: handle error
    let day = match &args.day[..] {
        "1" => advent2020::day_one::Solver::new(content),
        _ => panic!("Not implemented yet"),
    };

    let day = match day {
        Ok(val) => val,
        Err(err) => panic!("Could not set up day: {:?}", err),
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
