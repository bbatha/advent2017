const INPUT: &str = include_str!("input.txt");

fn part1(input: &[i64]) -> u64 {
    let mut input: Vec<_> = input.into();
    let mut steps = 0;
    let mut next = 0;
    while next < input.len() {
        steps += 1;
        let jmp = next as i64 + input[next];
        input[next] += 1;
        next = jmp as usize;
    }
    steps
}

#[test]
fn test_part1() {
    assert_eq!(5, part1(&[0, 3, 0, 1, -3]));
}

fn part2(input: &[i64]) -> u64 {
    let mut input: Vec<_> = input.into();
    let mut steps = 0;
    let mut next = 0;
    while next < input.len() {
        steps += 1;
        let jmp = next as i64 + input[next];
        if input[next] >= 3 {
            input[next] -= 1;
        } else {
            input[next] += 1;
        }
        next = jmp as usize;
    }
    steps
}

#[test]
fn test_part2() {
    assert_eq!(10, part2(&[0, 3, 0, 1, -3]));
}

fn main() {
    let input: Result<Vec<i64>, _> = INPUT.lines().map(std::str::FromStr::from_str).collect();
    let input = input.unwrap();

    println!("part1: {}", part1(&input));
    println!("part1: {}", part2(&input));
}
