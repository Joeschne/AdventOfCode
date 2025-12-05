use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug, Eq)]
enum Bound {
    Lower(u64),
    Upper(u64),
}

impl Bound {
    fn value(&self) -> u64 {
        match self {
            Bound::Lower(v) | Bound::Upper(v) => *v,
        }
    }
}

impl PartialEq for Bound {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Bound {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value()
            .cmp(&other.value())
            .then_with(|| match (self, other) {
                (Bound::Lower(_), Bound::Upper(_)) => Ordering::Less,
                (Bound::Upper(_), Bound::Lower(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            })
    }
}

impl PartialOrd for Bound {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn find_fresh_ingredients(input: String) {
    let (ranges, ids) = input
        .split_once("\n\n")
        .expect("two sections separated by blank line");

    let ranges = parse_ranges(ranges);
    let ids = ids.lines().map(|l| l.parse::<u64>().expect("valid int"));

    let ranges_normalized = normalize_ranges(ranges);

    println!("{:?}", ranges_normalized);

    let mut fresh_product_count = 0;

    for id in ids {
        let probe = Bound::Upper(id);

        match ranges_normalized.range(..=probe).next_back() {
            // lower, must be inside range
            Some(Bound::Lower(_)) => {
                fresh_product_count += 1;
            }

            // Upper, must be outside range unless exactly on edge
            Some(Bound::Upper(val)) => {
                if *val == id {
                    fresh_product_count += 1;
                }
            }

            None => {}
        }
    }

    println!("{fresh_product_count}");
}

pub(crate) fn find_fresh_ingredient_ids(input: String) {
    let (ranges, _) = input
        .split_once("\n\n")
        .expect("two sections separated by blank line");

    let ranges = parse_ranges(ranges);

    let ranges_normalized = normalize_ranges(ranges);

    println!("{:?}", ranges_normalized);

    let mut fresh_product_id_count = 0;
    let mut last_lower = None;

    for bound in ranges_normalized {
        match bound {
            Bound::Lower(v) => {
                last_lower = Some(v);
            }
            Bound::Upper(v) => {
                if let Some(lower) = last_lower {
                    fresh_product_id_count += v - lower + 1;
                }
                last_lower = None;
            }
        }
    }

    println!("{fresh_product_id_count}");
}

fn normalize_ranges(ranges: impl Iterator<Item = (Bound, Bound)>) -> BTreeSet<Bound> {
    let mut ranges_normalized = BTreeSet::new();

    for (lower, higher) in ranges {
        let removed: Vec<Bound> = ranges_normalized.range(lower..=higher).cloned().collect();

        for item in &removed {
            ranges_normalized.remove(item);
        }

        if removed.is_empty() {
            match ranges_normalized.range(..lower).next_back() {
                Some(Bound::Lower(_)) => {}
                // none: new lowest, insert directly
                // upper: new intermediary bound
                _ => {
                    ranges_normalized.insert(lower);
                }
            }
            match ranges_normalized.range(higher..).next() {
                Some(Bound::Upper(_)) => {}
                // none: new highest, insert directly
                // lower: new intermediary bound
                _ => {
                    ranges_normalized.insert(higher);
                }
            }
            continue;
        }

        if removed.iter().count() == 1 {
            match removed[0] {
                Bound::Lower(_) => ranges_normalized.insert(lower),
                Bound::Upper(_) => ranges_normalized.insert(higher),
            };
            continue;
        }

        let removed_low = removed.first().expect("at least 2 items");
        let removed_high = removed.last().expect("at least 2 items");

        match (removed_low, removed_high) {
            (Bound::Lower(_), Bound::Upper(_)) => {
                ranges_normalized.insert(lower);
                ranges_normalized.insert(higher);
            }
            (Bound::Lower(_), Bound::Lower(_)) => {
                ranges_normalized.insert(lower);
            }
            (Bound::Upper(_), Bound::Upper(_)) => {
                ranges_normalized.insert(higher);
            }
            // removed all intermediate, now new bigger range
            (Bound::Upper(_), Bound::Lower(_)) => {}
        }
    }

    ranges_normalized
}

fn parse_ranges(ranges: &str) -> impl Iterator<Item = (Bound, Bound)> {
    ranges.lines().map(|l| {
        let mut bounds = l.split('-');
        let lower = bounds
            .next()
            .expect("lower bound")
            .parse::<u64>()
            .expect("valid int");
        let upper = bounds
            .next()
            .expect("upper bound")
            .parse::<u64>()
            .expect("valid int");
        (Bound::Lower(lower), Bound::Upper(upper))
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        find_fresh_ingredients(TEST_INPUT.to_string());
    }

    #[test]
    fn test_part_2() {
        find_fresh_ingredient_ids(TEST_INPUT.to_string());
    }
}
