use crate::helpers;

pub(crate) fn solve_cephalopod_homework(input: String) {
    let mut rows: Vec<&str> = input.lines().collect();
    let sign_row = rows.pop().expect("non-empty input");

    // parse signs once
    let signs: Vec<u8> = sign_row
        .split_whitespace()
        .map(|s| s.as_bytes()[0])
        .collect();

    // initialize all column totals once
    let mut line_iter = rows.into_iter();
    let first_row = line_iter.next().expect("at least one numeric row");

    let mut column_totals: Vec<u64> = first_row
        .split_whitespace()
        .map(|t| t.parse::<u64>().expect("valid int"))
        .collect();

    for row in line_iter {
        for (column, val_str) in row.split_whitespace().enumerate() {
            let val = val_str.parse::<u64>().expect("valid int");
            match signs[column] {
                b'+' => column_totals[column] += val,
                b'*' => column_totals[column] *= val,
                _ => unreachable!(),
            }
        }
    }

    let result_total: u64 = column_totals.into_iter().sum();

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
                acc * 10 + helpers::ascii_to_digit(row[column]) as u64
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
