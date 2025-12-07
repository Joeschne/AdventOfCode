pub(crate) fn count_tachyon_beam_splits(input: String) {
    let mut tachyon_manifold = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|&c| c == b'^'));

    let initial_row = tachyon_manifold.next().expect("many lines");

    let num_cols = initial_row.len();
    let mid = num_cols / 2;

    let mut beam_map = vec![false; num_cols];
    beam_map[mid] = true;

    let mut split_count = 0;

    tachyon_manifold.skip(1).step_by(2).for_each(|row| {
        row.enumerate()
            .filter(|&(_, splitter_present)| splitter_present)
            .for_each(|(col, _)| {
                if beam_map[col] {
                    split_count += 1;
                    beam_map[col] = false;
                    beam_map[col - 1] = true;
                    beam_map[col + 1] = true;
                }
            });
    });

    println!("{split_count}");
}

pub(crate) fn count_quantum_tachyon_timelines(input: String) {
    let mut tachyon_manifold = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|&c| c == b'^'));

    let initial_row = tachyon_manifold.next().expect("many lines");

    let num_cols = initial_row.len();
    let mid = num_cols / 2;

    let mut beam_map = vec![0u64; num_cols];
    beam_map[mid] = 1;
    tachyon_manifold.skip(1).step_by(2).for_each(|row| {
        row.enumerate()
            .filter(|&(_, splitter_present)| splitter_present)
            .for_each(|(col, _)| {
                let timeline_count = beam_map[col];
                if timeline_count != 0 {
                    beam_map[col - 1] += timeline_count;
                    beam_map[col + 1] += timeline_count;
                    beam_map[col] = 0;
                }
            });
    });

    let total_timelines: u64 = beam_map.iter().sum();

    println!("{total_timelines}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1() {
        count_tachyon_beam_splits(TEST_INPUT.to_string());
    }

    #[test]
    fn test_part_2() {
        count_quantum_tachyon_timelines(TEST_INPUT.to_string());
    }
}
