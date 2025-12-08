use std::collections::HashSet;
use usable_floats::FloatWrapper;

#[cfg(test)]
const CONNECTIONS_TO_MAKE: usize = 10;
#[cfg(not(test))]
const CONNECTIONS_TO_MAKE: usize = 1000;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: FloatWrapper,
    y: FloatWrapper,
    z: FloatWrapper,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> FloatWrapper {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}

pub(crate) fn connect_closest_junction_boxes(input: String) {
    let coordinates = parse_coordinates(input);

    let mut distances = Vec::new();

    for (i, coordinate) in coordinates.iter().enumerate() {
        for neighbor in &coordinates[i + 1..] {
            let distance = coordinate.distance(neighbor);
            distances.push((distance, (coordinate, neighbor)));
        }
    }

    distances.sort_by_key(|(distance, _)| *distance);
    let mut distance_iter = distances.iter();

    let mut connection_sets: Vec<HashSet<&Coordinate>> = Vec::new();

    for _ in 0..CONNECTIONS_TO_MAKE {
        let mut inserted = Vec::new();

        let &(_, (neighbor_1, neighbor_2)) =
            distance_iter.next().expect("more than 1000 distances");

        for (i, connection_set) in connection_sets.iter_mut().enumerate() {
            // if either neihgobor is in, add the other one
            if connection_set.contains(neighbor_1) {
                connection_set.insert(neighbor_2);
                inserted.push(i);
            } else if connection_set.contains(neighbor_2) {
                connection_set.insert(neighbor_1);
                inserted.push(i);
            }
        }
        // new set
        if inserted.is_empty() {
            connection_sets.push(HashSet::from([neighbor_1, neighbor_2]));
        } else if inserted.len() > 1 {
            // merge all sets
            let first_set_index = inserted[0];
            for &other_set_index in &inserted[1..] {
                let other_set = connection_sets.swap_remove(other_set_index);
                for coord in other_set {
                    connection_sets[first_set_index].insert(coord);
                }
            }
        }
    }

    let mut component_sizes: Vec<usize> = connection_sets.iter().map(|s| s.len()).collect();
    component_sizes.sort_by(|a, b| b.cmp(a)); // descending

    let result: usize = component_sizes.iter().take(3).product();

    println!("{result}");
}

fn parse_coordinates(input: String) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            Coordinate {
                x: parts
                    .next()
                    .expect("three numbers")
                    .parse()
                    .expect("valid number"),
                y: parts
                    .next()
                    .expect("three numbers")
                    .parse()
                    .expect("valid number"),
                z: parts
                    .next()
                    .expect("three numbers")
                    .parse()
                    .expect("valid number"),
            }
        })
        .collect()
}

mod usable_floats {
    use std::{hash::Hash, str::FromStr};

    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    pub(super) struct FloatWrapper(f64);

    impl FromStr for FloatWrapper {
        type Err = std::num::ParseFloatError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(FloatWrapper(s.parse::<f64>()?))
        }
    }
    impl Eq for FloatWrapper {}
    impl Ord for FloatWrapper {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).expect("no NaN values")
        }
    }
    impl std::ops::Add for FloatWrapper {
        type Output = FloatWrapper;

        fn add(self, rhs: Self) -> Self::Output {
            FloatWrapper(self.0 + rhs.0)
        }
    }
    impl std::ops::Sub for FloatWrapper {
        type Output = FloatWrapper;

        fn sub(self, rhs: Self) -> Self::Output {
            FloatWrapper(self.0 - rhs.0)
        }
    }
    impl Hash for FloatWrapper {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.to_bits().hash(state);
        }
    }
    impl FloatWrapper {
        pub(super) fn powi(&self, n: i32) -> FloatWrapper {
            FloatWrapper(self.0.powi(n))
        }

        pub(super) fn sqrt(&self) -> FloatWrapper {
            FloatWrapper(self.0.sqrt())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_1() {
        connect_closest_junction_boxes(TEST_INPUT.to_string());
    }
}
