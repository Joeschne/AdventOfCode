pub(crate) fn solve_part_1(input: String) {
    let mut invalid_ids = Vec::new();

    let ranges = parse_ranges(&input);

    for range in ranges {
        let (lower, upper) = (
            range.0.parse::<u64>().expect("valid int"),
            range.1.parse::<u64>().expect("valid int"),
        );
        let mut current = lower;
        let mut same;
        while current <= upper {
            same = true;
            let s = current.to_string();
            let len = s.len();
            if len % 2 != 0 {
                current = next_power_of_10(current);
                continue;
            }
            let (left, right) = s.split_at(s.len() / 2);
            for (left, right) in left.chars().zip(right.chars()) {
                if left != right {
                    same = false;
                    break;
                }
            }
            if same {
                invalid_ids.push(current);
            }
            current += 1;
        }
    }

    println!("{}", invalid_ids.iter().sum::<u64>());
}

fn next_power_of_10(n: u64) -> u64 {
    10u64.pow((n as f64).log10().floor() as u32 + 1)
}

pub(crate) fn solve_part_2(input: String) {
    let mut invalid_ids = Vec::new();

    let ranges = parse_ranges(&input);

    for range in ranges {
        let (lower, upper) = (
            range.0.parse::<u64>().expect("valid int"),
            range.1.parse::<u64>().expect("valid int"),
        );
        for num in lower..=upper {
            let s = num.to_string();
            let len = s.len();

            let mut is_repeating = false;
            for pattern_len in 1..=len / 2 {
                if len % pattern_len != 0 {
                    continue;
                }

                let pattern = &s[..pattern_len];
                let mut matches = true;

                for start_index in (pattern_len..len).step_by(pattern_len) {
                    if &s[start_index..start_index + pattern_len] != pattern {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    is_repeating = true;
                    break;
                }
            }

            if is_repeating {
                invalid_ids.push(num);
            }
        }
    }

    println!("{}", invalid_ids.iter().sum::<u64>());
}

fn parse_ranges(input: &str) -> Vec<(&str, &str)> {
    input
        .split(',')
        .map(|range| {
            let mut bounds = range.split('-');
            (
                bounds.next().expect("lower bound"),
                bounds.next().expect("upper bound"),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        solve_part_1(TEST_INPUT.to_string());
    }

    #[test]
    fn test_part_2() {
        solve_part_2(TEST_INPUT.to_string());
    }
}
