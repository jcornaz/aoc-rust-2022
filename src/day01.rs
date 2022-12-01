type Output = u64;

pub fn part_1(input: &str) -> Output {
    let mut current_elf: u64 = 0;
    let mut max_elf : u64 = 0;
    for line in input.lines() {
        match line.parse::<u64>() {
            Ok(v) => current_elf += v,
            Err(_) => {
                current_elf = 0;
            }
        }
        if current_elf > max_elf {
            max_elf = current_elf;
        }
    }
    max_elf
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
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
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 0)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }
}
