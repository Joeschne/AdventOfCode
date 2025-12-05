// part 1
pub(crate) fn find_door_password(input: String) {
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

// part 2
#[allow(non_snake_case)]
pub(crate) fn find_door_password_method_0x434C49434B(input: String) {
    let mut zeros_found = 0;
    let mut current_position = 50;

    input.lines().for_each(|l| {
        let mut c = l.chars();
        let go_right = c.next() == Some('R');
        let amount: usize = c.collect::<String>().parse().expect("Should be a number");
        current_position = if go_right {
            zeros_found += (current_position + amount) / 100;
            (current_position + amount) % 100
        } else {
            // directly on zero, only full circles
            if current_position == 0 {
                zeros_found += amount / 100;
            } else if amount >= current_position {
                // at least one passed, rest full circles
                zeros_found += 1 + (amount - current_position) / 100;
            }
            (current_position + 100 - (amount % 100)) % 100
        }
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
        find_door_password(TEST_INPUT.to_string());
    }

    #[test]
    fn test_day_2() {
        find_door_password_method_0x434C49434B(TEST_INPUT.to_string());
    }
}
