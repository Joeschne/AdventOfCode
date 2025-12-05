use std::collections::{HashMap, HashSet, VecDeque};

type Position = (usize, usize);

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

// part 1
pub(crate) fn find_accessible_paper_stacks(input: String) {
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
            handle_neighbors((row, column), (rows, columns), &paper_map, |_| {
                adjacent_rolls += 1;
            });
            if adjacent_rolls < 4 {
                accessible_stacks += 1;
            }
        }
    }

    println!("{accessible_stacks}");
}

// part 2
pub(crate) fn remove_accessible_paper_stacks(input: String) {
    let paper_map: Vec<Vec<bool>> = input
        .lines()
        .map(|l| { l.chars().map(|c| c == '@') }.collect())
        .collect();

    let mut neighbor_map: HashMap<Position, HashSet<Position>> = HashMap::new();

    let rows = paper_map.len();
    let columns = paper_map[0].len();

    for row in 0..rows {
        for column in 0..columns {
            if !paper_map[row][column] {
                continue;
            }
            let neighbors = neighbor_map.entry((row, column)).or_default();
            handle_neighbors((row, column), (rows, columns), &paper_map, |neighbor| {
                neighbors.insert(*neighbor);
            });
        }
    }

    let removed_stack_count = prune_k_core(&mut neighbor_map, 4);

    println!("{removed_stack_count}");
}

// copy-paste from AI - not implemented by me
fn prune_k_core(neighbor_map: &mut HashMap<Position, HashSet<Position>>, k: usize) -> usize {
    let mut queue = VecDeque::new();

    // 1. Initialize queue with all nodes that are already < k
    for (&pos, neighbors) in neighbor_map.iter() {
        if neighbors.len() < k {
            queue.push_back(pos);
        }
    }

    let mut removed_count = 0;

    // 2. Process queue
    while let Some(pos) = queue.pop_front() {
        // Node might have been removed already due to a previous neighbor
        let neighbors = match neighbor_map.remove(&pos) {
            Some(neighbors) => neighbors,
            None => continue,
        };

        removed_count += 1;

        // 3. For each neighbor, remove this node and check if its degree dropped below k
        for neighbor_pos in neighbors {
            if let Some(neigh_neighbors) = neighbor_map.get_mut(&neighbor_pos) {
                neigh_neighbors.remove(&pos);

                if neigh_neighbors.len() == k - 1 {
                    // It just dropped below k, schedule it
                    queue.push_back(neighbor_pos);
                }
            }
        }
    }

    removed_count
}

fn handle_neighbors(
    position: Position,
    map_size: Position,
    paper_map: &Vec<Vec<bool>>,
    mut found_neighbor_predicate: impl FnMut(&Position),
) {
    let (rows, columns) = map_size;
    for (x_delta, y_delta) in DIRECTIONS {
        let neighbor_x = position.0 as isize + x_delta;
        let neighbor_y = position.1 as isize + y_delta;
        if neighbor_x < 0 || neighbor_y < 0 {
            continue;
        }
        let neighbor_x = neighbor_x as usize;
        let neighbor_y = neighbor_y as usize;
        if neighbor_x >= rows || neighbor_y >= columns {
            continue;
        }
        if paper_map[neighbor_x][neighbor_y] {
            found_neighbor_predicate(&(neighbor_x, neighbor_y));
        }
    }
}

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
        find_accessible_paper_stacks(TEST_INPUT.to_string());
    }

    #[test]
    fn test_part_2() {
        remove_accessible_paper_stacks(TEST_INPUT.to_string());
    }
}
