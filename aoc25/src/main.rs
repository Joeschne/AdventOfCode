use aoc25::parser::Cli;
use clap::Parser;

fn main() {
    println!();
    let cli = Cli::parse();

    let file_postfix = if cli.separate_input_files {
        format!("{}-{}", cli.day, cli.part)
    } else {
        format!("{}", cli.day)
    };

    let file_name = format!("./input-files/day{}.txt", file_postfix);

    let input = std::fs::read_to_string(&file_name);

    let input = input.unwrap_or_else(|e| {
        eprintln!("Error reading input file at {file_name}: {e}");
        std::process::exit(1);
    });

    eprintln!("Ho ho ho! It be the jolly season of codin'");
    eprintln!("Let's try day {} part {}", cli.day, cli.part);
    if cli.time_execution {
        eprintln!("Woouw. Let's see how fast this sled runs!");
    }
    eprintln!();

    aoc25::day_solutions::run_day_part(cli.day, cli.part, input, cli.time_execution);
}
