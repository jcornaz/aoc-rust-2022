type Output = u64;

pub fn part_1(input: &str) -> Output {
    0
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

fn find_misplaced_item(rucksack: &str) -> char {
    'p'
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
    #[ignore = "not implemented"]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 16)]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 157)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
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
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 'p')]
    fn should_find_misplaced_item(#[case] rucksack: &str, #[case] expected: char) {
        assert_eq!(find_misplaced_item(rucksack), expected);
    }
}
