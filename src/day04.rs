use std::ops::RangeInclusive;

type Output = usize;

pub fn part_1(input: &str) -> Output {
    input.lines().filter(|s| is_full_overlap(s)).count()
}

pub fn part_2(input: &str) -> Output {
    input.lines().filter(|s| is_overlap(s)).count()
}

fn parse_line(input: &str) -> Option<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (left, right) = input.split_once(',')?;
    let left = parse_range(left)?;
    let right = parse_range(right)?;
    Some((left, right))
}

fn parse_range(input: &str) -> Option<RangeInclusive<u32>> {
    let (min, max) = input.split_once('-')?;
    Some(min.parse::<u32>().ok()?..=max.parse::<u32>().ok()?)
}

fn is_overlap(input: &str) -> bool {
    let Some((left, right)) = parse_line(input) else { return false };
    left.contains(right.start()) || left.contains(right.end()) || right.contains(left.start())
}

fn is_full_overlap(input: &str) -> bool {
    let Some((left, right)) = parse_line(input) else { return false };
    (left.contains(right.start()) && left.contains(right.end()))
        || (right.contains(left.start()) && right.contains(left.end()))
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::*;

    const EXAMPLE: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#;

    const INPUT: &str = include_str!("day04/input.txt");

    #[rstest]
    #[case::empty("", 0)]
    #[case("2-8,3-7", 1)]
    #[case::example(EXAMPLE, 2)]
    #[case::input(INPUT, 580)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[case::example(EXAMPLE, 4)]
    #[case::input(INPUT, 895)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }

    #[rstest]
    #[case("1-2", 1..=2)]
    fn should_parse_range(#[case] input: &str, #[case] expected: RangeInclusive<u32>) {
        assert_eq!(parse_range(input), Some(expected))
    }

    #[rstest]
    #[case("2-4,6-8", false)]
    #[case("2-3,4-5", false)]
    #[case("5-7,7-9", false)]
    #[case("2-8,3-7", true)]
    #[case("6-6,4-6", true)]
    #[case("2-6,4-8", false)]
    fn should_reckognize_full_overlap(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_full_overlap(input), expected);
    }

    #[rstest]
    #[case("2-4,6-8", false)]
    #[case("2-3,4-5", false)]
    #[case("5-7,7-9", true)]
    #[case("2-8,3-7", true)]
    #[case("6-6,4-6", true)]
    #[case("2-6,4-8", true)]
    #[case("3-4,1-5", true)]
    fn should_reckognize_partial_overlap(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_overlap(input), expected);
    }
}
