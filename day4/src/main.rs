fn part1(phrase: &str) -> bool {
    let mut words = std::collections::BTreeSet::new();
    for word in phrase.split_whitespace() {
        if words.get(word).is_some() {
            return false
        };
        words.insert(word);
    }

    true
}

#[test]
fn test_is_valid() {
    assert!(part1("aa bb cc dd ee"));
    assert!(!part1("aa bb cc dd aa"));
    assert!(part1("aa bb cc dd aaa"));
}

fn part2(phrase: &str) -> bool {
    let mut words = std::collections::BTreeSet::new();
    for word in phrase.split_whitespace() {
        let mut word: Vec<_> = word.chars().collect();
        word.sort();
        if words.get(&word).is_some() {
            return false
        };
        words.insert(word);
    }

    true

}

#[test]
fn test_part2() {
    assert!(part2("abcde fghij"));
    assert!(!part2("abcde xyz ecdab"));
    assert!(part2("a ab abc abd abf abj"));
    assert!(part2("iiii oiii ooii oooi oooo"));
    assert!(!part2("oiii ioii iioi iiio"));
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let phrases = INPUT.lines().filter(|&x| part1(x)).count();
    println!("{} part1", phrases);
    let phrases = INPUT.lines().filter(|&x| part2(x)).count();
    println!("{} part2", phrases);
}
