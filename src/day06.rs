use std::collections::HashSet;

type Output = usize;

pub fn part_1(input: &str) -> Output {
    length_until_signal(4, input).unwrap()
}

pub fn part_2(input: &str) -> Output {
    length_until_signal(14, input).unwrap()
}

fn length_until_signal(n: usize, input: &str) -> Option<usize> {
    let mut set = HashSet::<char>::with_capacity(n);
    input
        .char_indices()
        .map(|(i, _)| (i, &input[i..(i + n)]))
        .find(|(_, s)| {
            set.clear();
            set.extend(s.chars());
            set.len() == n
        })
        .map(|(index, _)| index + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("day06/input.txt");

    #[rstest]
    #[case("abcd", 4)]
    #[case("aabcd", 5)]
    #[case("aabcde", 5)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case::input(INPUT, 1766)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case::input(INPUT, 2383)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }
}
