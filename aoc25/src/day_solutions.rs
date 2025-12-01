mod day01;

pub fn run_day_part(day: usize, part: u8, input: String) {
    match (day, part) {
        (1, 1) => day01::solve_part_1(input),
        (1, 2) => day01::solve_part_2(input),
        _ => {
            eprintln!("Day {} part {} is not (yet) implemented.", day, part);
            std::process::exit(1);
        }
    }
}
