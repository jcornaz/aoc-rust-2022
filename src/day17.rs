type Output = u64;

pub fn part_1(input: &str) -> Output {
    input.parse().unwrap()
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

struct Simulation {
    stream: Vec<char>,

    /// Row major (bottom to up)
    cells: Vec<Vec<bool>>,
}

impl Simulation {
    fn new(stream: &str) -> Self {
        Self {
            stream: stream.chars().collect(),
            cells: Vec::new(),
        }
    }

    fn simulate(&mut self, rock_count: u16) {
        (0..(rock_count as usize))
            .map(|i| SHAPES[i % SHAPES.len()])
            .for_each(|shape| self.drop_shape(shape));
    }

    fn drop_shape(&mut self, shape: Shape) {
        let mut x = 2;
        let mut y = self.height() + 3;
        while self.can_move_down((x, y), shape) {
            y -= 1;
        }
        self.stop((x, y), shape);
    }

    fn stop(&mut self, (x, y): (usize, usize), shape: Shape) {
        self.cells
            .resize_with(self.cells.len().max(shape.height() + y), Vec::new);
    }

    fn can_move_down(&self, (x, y): (usize, usize), shape: Shape) -> bool {
        y > self.height()
    }

    fn height(&self) -> usize {
        self.cells.len()
    }
}

const SHAPES: [Shape; 5] = [
    Shape::HorizontalLine,
    Shape::Plus,
    Shape::Corner,
    Shape::VerticalLine,
    Shape::Square,
];

#[derive(Debug, Copy, Clone)]
enum Shape {
    HorizontalLine,
    Plus,
    Corner,
    VerticalLine,
    Square,
}

impl Shape {
    fn height(self) -> usize {
        match self {
            Shape::HorizontalLine => 1,
            Shape::Corner | Shape::Plus => 3,
            Shape::Square => 2,
            Shape::VerticalLine => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    const INPUT: &str = include_str!("day17/input.txt");

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 3068)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[case(">", 1, 1)]
    #[case(">", 2, 4)]
    #[case(">", 3, 7)]
    #[case(">", 4, 11)]
    #[case(">", 5, 13)]
    #[case(">", 6, 14)]
    #[case(EXAMPLE, 1, 1)]
    #[case(EXAMPLE, 2, 4)]
    // #[case(EXAMPLE, 3, 6)]
    fn should_simulate_n_rocks(
        #[case] stream: &str,
        #[case] rock_count: u16,
        #[case] expected_height: usize,
    ) {
        let mut simulation = Simulation::new(stream);
        simulation.simulate(rock_count);
        assert_eq!(simulation.height(), expected_height);
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
