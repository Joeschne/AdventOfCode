pub(crate) fn solve_cephalopod_homework(input: String) {
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let sign_row = &rows[num_rows - 1];

    let mut result_total: u64 = 0;

    for column in 0..num_cols {
        let sign = sign_row[column].as_bytes()[0];

        // initial value
        let mut column_total = rows[0][column].parse::<u64>().expect("valid int");

        // all rows exclusive first one(initial) and last one (signs)
        for row in 1..(num_rows - 1) {
            let val = rows[row][column].parse::<u64>().expect("valid int");
            match sign {
                b'+' => column_total += val,
                b'*' => column_total *= val,
                _ => unreachable!(),
            }
        }

        result_total += column_total;
    }

    println!("{result_total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_1() {
        solve_cephalopod_homework(TEST_INPUT.to_string());
    }
}
