use std::collections::HashSet;

type Output = u64;

pub fn part_1(input: &str) -> Output {
    let mut droplets = HashSet::<(i32, i32, i32)>::new();
    let mut count = 0;
    input
        .lines()
        .map(|l| l.splitn(3, ',').collect::<Vec<_>>())
        .map(|v| {
            (
                v[0].parse().unwrap(),
                v[1].parse().unwrap(),
                v[2].parse().unwrap(),
            )
        })
        .for_each(|pos| {
            for side in sides(pos) {
                if droplets.contains(&side) {
                    count -= 1;
                } else {
                    count += 1;
                }
            }
            droplets.insert(pos);
        });
    count
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

fn sides((x, y, z): (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
    "#;

    const INPUT: &str = include_str!("day18/input.txt");

    #[rstest]
    #[case("1,1,1", 6)]
    #[case("1,1,1\n2,1,1", 10)]
    #[case("2,1,1\n1,1,1", 10)]
    #[case("1,1,1\n1,2,1", 10)]
    #[case("1,2,1\n1,1,1", 10)]
    #[case("1,1,1\n1,1,2", 10)]
    #[case("1,1,2\n1,1,1", 10)]
    #[case::example(EXAMPLE, 64)]
    #[case::input(INPUT, 4340)]
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
