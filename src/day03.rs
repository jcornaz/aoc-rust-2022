use itertools::Itertools;

type Output = u64;

pub fn part_1(input: &str) -> Output {
    input
        .lines()
        .filter_map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            find_item_in_all(&[comp1, comp2])
        })
        .map(score_of)
        .sum()
}

pub fn part_2(input: &str) -> Output {
    let mut sum = 0;
    for group in &input.lines().chunks(3) {
        let group: Vec<_> = group.collect();
        let Some(badge) = find_item_in_all(&group) else { continue };
        sum += score_of(badge);
    }
    sum
}

fn find_item_in_all(sacks: &[&str]) -> Option<char> {
    sacks
        .first()?
        .chars()
        .find(|&c| sacks.iter().skip(1).all(|s| s.contains(c)))
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

    const SACK_EXAMPLE_1: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
    "#;

    const SACK_EXAMPLE_2: &str = r#"
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

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
    #[case("", 0)]
    #[case("a\na\na", 1)]
    #[case("a\na\na\nb\nb\nb", 3)]
    #[case(SACK_EXAMPLE_1, 18)]
    #[case(SACK_EXAMPLE_2, 52)]
    #[case::example(EXAMPLE, 70)]
    #[case::input(INPUT, 2276)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
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

    #[rstest]
    #[case(&["ab", "bc"], 'b')]
    #[case(&["vJrwpWtwJgWr", "hcsFMMfFFhFp"], 'p')]
    #[case(&["ab", "ac", "ad"], 'a')]
    #[case(&["abc", "acd", "czy"], 'c')]
    fn should_find_badge(#[case] sacks: &[&str], #[case] expected: char) {
        assert_eq!(find_item_in_all(&sacks), Some(expected));
    }
}
