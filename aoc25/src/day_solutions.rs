mod day01;
mod day02;
mod day03;
mod day04;

pub fn run_day_part(day: usize, part: u8, input: String, time_execution: bool) {
    let start_time = if time_execution {
        Some(std::time::Instant::now())
    } else {
        None
    };
    match (day, part) {
        (1, 1) => day01::solve_part_1(input),
        (1, 2) => day01::solve_part_2(input),
        (2, 1) => day02::solve_part_1(input),
        (2, 2) => day02::solve_part_2(input),
        (3, 1) => day03::solve_part_1(input),
        (3, 2) => day03::solve_part_2(input),
        (4, 1) => day04::solve_part_1(input),
        (4, 2) => day04::solve_part_2(input),
        _ => {
            eprintln!("Day {} part {} is not (yet) implemented.", day, part);
            std::process::exit(1);
        }
    }
    if let Some(start) = start_time {
        let duration = start.elapsed();
        eprintln!("\nExecution time: {:?}", duration);
    }
}
