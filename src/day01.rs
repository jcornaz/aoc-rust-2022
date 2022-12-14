type Output = u64;

pub fn part_1(input: &str) -> Output {
    parse(input).max().unwrap_or_default()
}

pub fn part_2(input: &str) -> Output {
    let mut elves: Vec<_> = parse(input).collect();
    elves.sort();
    elves.into_iter().rev().take(3).sum()
}

fn parse(input: &str) -> impl Iterator<Item = Output> + '_ {
    input.split("\n\n").map(|elf| {
        elf.lines()
            .filter_map(|fruit| fruit.parse::<Output>().ok())
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
    "#;

    const INPUT: &str = include_str!("day01/input.txt");

    #[rstest]
    #[case("200\n300", 500)]
    #[case("200\n300\n\n500", 500)]
    #[case("200\n300\n\n600", 600)]
    #[case::example(EXAMPLE, 24000)]
    #[case::input(INPUT, 69528)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[case("200\n300\n\n500", 1000)]
    #[case("200\n300\n\n500\n\n100\n\n600", 1600)]
    #[case::example(EXAMPLE, 45000)]
    #[case::input(INPUT, 206152)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }
}
