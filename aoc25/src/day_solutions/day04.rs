use std::collections::{HashMap, HashSet};

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

pub(crate) fn solve_part_2(input: String) {
    let paper_map: Vec<Vec<bool>> = input
        .lines()
        .map(|l| { l.chars().map(|c| c == '@') }.collect())
        .collect();

    let mut neighbor_map: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    let rows = paper_map.len();
    let columns = paper_map[0].len();

    for row in 0..rows {
        for column in 0..columns {
            if !paper_map[row][column] {
                continue;
            }
            let neighbors = neighbor_map.entry((row, column)).or_default();
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
                    neighbors.insert((neighbor_x, neighbor_y));
                }
            }
        }
    }

    let mut removed_stacks = Vec::new();
    let mut removed_stack_count = 0;
    loop {
        for (&position, neighbors) in neighbor_map.iter() {
            if neighbors.len() < 4 {
                removed_stacks.push(position)
            };
        }
        if removed_stacks.is_empty() {
            break;
        }
        removed_stack_count += removed_stacks.len();
        for &position in &removed_stacks {
            neighbor_map.remove(&position);
            for neighbors in neighbor_map.values_mut() {
                neighbors.remove(&position);
            }
        }
        removed_stacks.clear();
    }

    println!("{removed_stack_count}");
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

    #[test]
    fn test_part_2() {
        solve_part_2(TEST_INPUT.to_string());
    }
}
