pub(crate) fn solve_part_1(input: String) {
    let paper_map: Vec<Vec<bool>> = input
        .lines()
        .map(|l| { l.chars().map(|c| c == '@') }.collect())
        .collect();

    let mut accessible_stacks = 0;

    let rows = paper_map.len();
    let columns = paper_map[0].len();

    for row in 0..rows {
        for column in 0..columns {
            if !paper_map[row][column] {
                continue;
            }
            let mut adjacent_rolls = 0;
            for (x_delta, y_delta) in DIRECTIONS {
                let neighbor_x = row as isize + x_delta;
                let neighbor_y = column as isize + y_delta;
                if neighbor_x < 0 || neighbor_y < 0 {
                    continue;
                }
                let neighbor_x = neighbor_x as usize;
                let neighbor_y = neighbor_y as usize;
                if neighbor_x >= rows || neighbor_y >= columns {
                    continue;
                }
                if paper_map[neighbor_x][neighbor_y] {
                    adjacent_rolls += 1;
                }
            }
            if adjacent_rolls < 4 {
                accessible_stacks += 1;
            }
        }
    }

    println!("{accessible_stacks}");
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        solve_part_1(TEST_INPUT.to_string());
    }
}
