use std::str::FromStr;

fn redistribute(state: &mut [u8]) {
    let (mut i, bank) = state.iter()
        .cloned()
        .enumerate()
        .max_by_key(|&(i, val)| (val, -(i as isize))) // use index to break ties
        .unwrap();

    state[i] = 0;
    for _ in (0..bank).rev() {
        if i < state.len() - 1 {
            i += 1;
        } else {
            i = 0;
        }
        state[i] += 1;
    }
}

fn part1(input: &mut [u8]) -> u64 {
    if input.is_empty() {
        return 0;
    }

    let mut seen = std::collections::BTreeSet::new();
    while seen.insert(input.to_owned()) {
        redistribute(input);
    }

    seen.len() as u64
}

#[test]
fn test() {
    // part1
    assert_eq!(5, part1(&mut [0, 2, 7, 0]));
    // part2
    assert_eq!(4, part1(&mut [2, 4, 1, 2]));
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Result<Vec<u8>, _> = INPUT.split_whitespace().map(FromStr::from_str).collect();
    let mut input = input.unwrap();
    println!("Part 1: {}", part1(&mut input));
    println!("Part 2: {}", part1(&mut input));
}
