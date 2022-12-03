use std::collections::HashSet;

type Output = u64;

pub fn part_1(input: &str) -> Output {
    input
        .lines()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            let item = find_misplaced_item(comp1, comp2);
            score_of(item)
        })
        .sum()
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

fn find_misplaced_item(compartment1: &str, compartment2: &str) -> char {
    let compartment1: HashSet<char> = compartment1.chars().collect();
    compartment2
        .chars()
        .find(|c| compartment1.contains(c))
        .unwrap_or_default()
}

fn score_of(item: char) -> Output {
    if ('a'..='z').contains(&item) {
        item as Output - 'a' as Output + 1
    } else if ('A'..='Z').contains(&item) {
        item as Output - 'A' as Output + 27
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    const INPUT: &str = include_str!("day03/input.txt");

    #[rstest]
    #[case("", 0)]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 16)]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38)]
    #[case::example(EXAMPLE, 157)]
    #[case::input(INPUT, 7742)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 0)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }

    #[rstest]
    #[case("ab", "bc", 'b')]
    #[case("vJrwpWtwJgWr", "hcsFMMfFFhFp", 'p')]
    fn should_find_misplaced_item(
        #[case] compartment1: &str,
        #[case] compartment2: &str,
        #[case] expected: char,
    ) {
        assert_eq!(find_misplaced_item(compartment1, compartment2), expected);
    }

    #[rstest]
    #[case('a', 1)]
    #[case('b', 2)]
    #[case('z', 26)]
    #[case('A', 27)]
    #[case('B', 28)]
    fn should_find_score(#[case] item: char, #[case] expected: Output) {
        assert_eq!(score_of(item), expected);
    }
}
