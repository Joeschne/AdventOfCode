pub(crate) fn solve_part_1(input: String) {
    let mut invalid_ids = Vec::new();

    let ranges = input.split(',').map(|range| {
        let mut bounds = range.split('-');
        (
            bounds.next().expect("lower bound"),
            bounds.next().expect("upper bound"),
        )
    });

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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        solve_part_1(TEST_INPUT.to_string());
    }
}
