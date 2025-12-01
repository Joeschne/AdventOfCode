pub(crate) fn solve_part_1(input: String) {
    let mut zeros_found = 0;
    let mut current_position = 50;

    input.lines().for_each(|l| {
        let mut c = l.chars();
        let go_right = c.next() == Some('R');
        let amount: usize = c.collect::<String>().parse().expect("Should be a number");
        current_position = if go_right {
            (current_position + amount) % 100
        } else {
            (current_position + 100 - (amount % 100)) % 100
        };
        zeros_found += (current_position == 0) as usize;
    });

    println!("{zeros_found}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_day_1() {
        solve_part_1(TEST_INPUT.to_string());
    }
}
