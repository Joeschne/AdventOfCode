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

pub(crate) fn solve_cephalopod_homework_properly(input: String) {
    let rows: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let num_cols = rows[0].len();
    let number_row_count = rows.len() - 1;

    let sign_row = rows[number_row_count];
    let mut sign = sign_row[0];
    let mut multiplication_batch_total = None;
    let mut result_total = 0;

    for column in 0..num_cols {
        let column_number = rows.iter().take(number_row_count).fold(0, |acc, &row| {
            if row[column].is_ascii_digit() {
                acc * 10 + (row[column] - b'0') as u64
            } else {
                acc
            }
        });
        // no column is ever all zeros
        if column_number == 0 {
            sign = sign_row[column + 1];
            result_total += multiplication_batch_total.unwrap_or(0);
            multiplication_batch_total = None;
            continue;
        }
        match sign {
            b'+' => {
                result_total += column_number;
            }
            b'*' => {
                if let Some(batch_total) = multiplication_batch_total.as_mut() {
                    *batch_total *= column_number;
                } else {
                    multiplication_batch_total = Some(column_number);
                }
            }
            _ => unreachable!(),
        }
    }

    // remaining multiplication
    result_total += multiplication_batch_total.unwrap_or(0);

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

    #[test]
    fn test_part_2() {
        solve_cephalopod_homework_properly(TEST_INPUT.to_string());
    }
}
