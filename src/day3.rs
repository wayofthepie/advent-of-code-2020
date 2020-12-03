pub mod part_one {
    pub fn solution(terrain: &str) -> usize {
        terrain
            .lines()
            .map(|line| line.as_bytes())
            .enumerate()
            .filter(|(i, bytes)| bytes[(i * 3) % bytes.len()] == b'#')
            .count()
    }
}

pub mod part_two {
    pub fn solution(terrain: &str) -> usize {
        terrain
            .lines()
            .map(|line| line.as_bytes())
            .enumerate()
            .fold([0 as usize; 5], |[v, w, x, y, z], (i, bytes)| {
                let len = bytes.len();
                let is_tree = |index| (bytes[index % len] == b'#') as usize;
                let z = z + (i != 0 && i % 2 == 0 && bytes[(i / 2) % len] == b'#') as usize;
                [
                    v + is_tree(i),
                    w + is_tree(i * 3),
                    x + is_tree(i * 5),
                    y + is_tree(i * 7),
                    z,
                ]
            })
            .iter()
            .product()
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn part_one_should_give_right_answer() {
        let terrain = std::fs::read_to_string("resources/day3.txt").unwrap();
        assert_eq!(218, part_one::solution(&terrain));
    }

    #[test]
    fn part_two_should_give_right_answer() {
        let terrain = std::fs::read_to_string("resources/day3.txt").unwrap();
        assert_eq!(3847183340, part_two::solution(&terrain));
    }

    #[test]
    fn part_two_should_solve_example() {
        let example = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        assert_eq!(336, part_two::solution(&example));
    }
}
