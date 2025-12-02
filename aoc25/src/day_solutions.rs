mod day01;
mod day02;

pub fn run_day_part(day: usize, part: u8, input: String) {
    match (day, part) {
        (1, 1) => day01::solve_part_1(input),
        (1, 2) => day01::solve_part_2(input),
        (2, 1) => day02::solve_part_1(input),
        _ => {
            eprintln!("Day {} part {} is not (yet) implemented.", day, part);
            std::process::exit(1);
        }
    }
}
