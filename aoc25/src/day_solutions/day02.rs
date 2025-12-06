// part 1
pub(crate) fn find_invalid_gift_shop_ids(input: String) {
    let mut invalid_id_sum = 0;

    let ranges = parse_ranges(&input);

    for range in ranges {
        let (lower, upper) = range;
        let mut current = lower;
        let mut same;
        while current <= upper {
            let s = current.to_string();
            let len = s.len();
            if len % 2 != 0 {
                // skip to next even length
                current = next_power_of_10(current) + 1;
                continue;
            } else {
                let (left, right) = s.split_at(len / 2);
                // each char in both halves equal
                same = left.bytes().zip(right.bytes()).all(|(l, r)| l == r);
            }
            if same {
                invalid_id_sum += current;
            }
            current += 1;
        }
    }

    println!("{}", invalid_id_sum);
}

fn next_power_of_10(n: u64) -> u64 {
    10u64.pow((n as f64).log10().floor() as u32 + 1)
}

// part 2
pub(crate) fn find_more_invalid_ids(input: String) {
    let mut invalid_id_sum = 0;

    let ranges = parse_ranges(&input);

    for range in ranges {
        let (lower, upper) = range;
        for num in lower..=upper {
            let s = num.to_string();
            let len = s.len();

            let is_repeating = (1..=len / 2).any(|pattern_len| {
                // pattern must divide length evenly
                if len % pattern_len != 0 {
                    return false;
                }
                let pattern = &s[..pattern_len];
                (pattern_len..len)
                    .step_by(pattern_len)
                    .all(|start| &s[start..start + pattern_len] == pattern)
            });

            if is_repeating {
                invalid_id_sum += num;
            }
        }
    }

    println!("{}", invalid_id_sum);
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|range| {
            let mut bounds = range.split('-');
            (
                bounds
                    .next()
                    .expect("lower bound")
                    .parse::<u64>()
                    .expect("valid int"),
                bounds
                    .next()
                    .expect("upper bound")
                    .parse::<u64>()
                    .expect("valid int"),
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
        find_invalid_gift_shop_ids(TEST_INPUT.to_string());
    }

    #[test]
    fn test_part_2() {
        find_more_invalid_ids(TEST_INPUT.to_string());
    }
}
