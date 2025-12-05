// part 1
pub(crate) fn calculate_total_max_joltage(input: String) {
    let banks = parse_banks(&input);
    let mut total_joltage = 0;
    for mut bank in banks {
        total_joltage += calculate_max_bank_joltage_two_batteries(&mut bank);
    }

    println!("{total_joltage}");
}

fn calculate_max_bank_joltage_two_batteries(mut bank: impl Iterator<Item = u32>) -> u32 {
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

// part 2
pub(crate) fn calculate_total_max_joltage_overclock(input: String) {
    let banks = parse_banks(&input);
    let mut total_joltage = 0;
    for bank in banks {
        total_joltage += calculate_max_bank_joltage::<12>(bank.collect::<Vec<u32>>());
    }
    println!("{total_joltage}");
}

fn calculate_max_bank_joltage<const BATTERY_COUNT: usize>(bank: Vec<u32>) -> u128 {
    let bank_size = bank.len();
    assert!(bank_size >= BATTERY_COUNT);
    // (battery, index): initialize index with -1
    let mut on_batteries = [(0u32, usize::MAX); BATTERY_COUNT];

    for (i, &candidate) in bank.iter().enumerate() {
        for j in 0..BATTERY_COUNT {
            // previous index must be < i for order to hold
            if j > 0 && on_batteries[j - 1].1 >= i {
                continue;
            }
            let (battery, _) = on_batteries[j];
            if candidate <= battery {
                continue;
            }
            let suffix_length = BATTERY_COUNT - j;
            // enough digits left to fill j..COUNT-1
            if bank_size - i < suffix_length {
                continue;
            }

            // replace suffix with contiguous slice starting at i
            let src = &bank[i..i + suffix_length];
            for offset in 0..suffix_length {
                on_batteries[j + offset] = (src[offset], i + offset);
            }
            break;
        }
    }

    on_batteries
        .iter()
        .map(|(d, _)| *d as u128)
        .fold(0u128, |acc, d| acc * 10 + d)
}

fn parse_banks(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).expect("valid digit")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_solve_part_1() {
        calculate_total_max_joltage(TEST_INPUT.to_string());
    }

    #[test]
    fn test_solve_part_2() {
        calculate_total_max_joltage_overclock(TEST_INPUT.to_string());
    }
}
