mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn run_day_part(day: usize, part: u8, input: String, time_execution: bool) {
    let start_time = if time_execution {
        Some(std::time::Instant::now())
    } else {
        None
    };
    match (day, part) {
        (1, 1) => day01::find_door_password(input),
        (1, 2) => day01::find_door_password_method_0x434C49434B(input),
        (2, 1) => day02::find_invalid_gift_shop_ids(input),
        (2, 2) => day02::find_more_invalid_ids(input),
        (3, 1) => day03::calculate_total_max_joltage(input),
        (3, 2) => day03::calculate_total_max_joltage_overclock(input),
        (4, 1) => day04::find_accessible_paper_stacks(input),
        (4, 2) => day04::remove_accessible_paper_stacks(input),
        (5, 1) => day05::find_fresh_ingredients(input),
        (5, 2) => day05::find_fresh_ingredient_ids(input),
        (6, 1) => day06::solve_cephalopod_homework(input),
        (6, 2) => day06::solve_cephalopod_homework_properly(input),
        (7, 1) => day07::count_tachyon_beam_splits(input),
        (7, 2) => day07::count_quantum_tachyon_timelines(input),
        (8, 1) => day08::connect_closest_junction_boxes(input),
        (8, 2) => day08::create_big_junction_box_circuit(input),
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
