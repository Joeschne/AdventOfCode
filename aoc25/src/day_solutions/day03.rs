pub(crate) fn solve_part_1(input: String) {
    let banks = parse_banks(&input);
    let mut total_joltage = 0;
    for mut bank in banks {
        total_joltage += calculate_max_bank_joltage(&mut bank);
    }

    println!("{total_joltage}");
}

fn calculate_max_bank_joltage(mut bank: impl Iterator<Item = u32>) -> u32 {
    let first = bank.next().expect("at least one digit");
    let second = bank.next().expect("at least two digits");

    let mut best_ten = first;
    let mut best_one = second;

    let mut best_tens_so_far = if second > first { second } else { first };

    for battery in bank {
        // last one we added was best tens or battery is better than best one
        if best_tens_so_far > best_ten || battery > best_one {
            best_ten = best_tens_so_far;
            best_one = battery;
        }

        if battery > best_tens_so_far {
            best_tens_so_far = battery;
        }
    }

    best_ten * 10 + best_one
}

fn parse_banks(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).expect("valid digit")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "110
00120
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_solve_part_1() {
        solve_part_1(TEST_INPUT.to_string());
    }
}
