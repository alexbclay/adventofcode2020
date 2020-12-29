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

    // TODO: handle error
    let day = match &args.day[..] {
        "1" => advent2020::day_one::Solver::new(content),
        _ => panic!("Not implemented yet"),
    };

    println!("Solution Part 1: {}", day.part_one());
    println!("Solution Part 2: {}", day.part_two());
}
