pub mod part_one {
    pub fn solution(input: &str) -> Result<isize, Box<dyn std::error::Error>> {
        let mut acc = 0;
        let mut program: Vec<Option<(&str, &str)>> =
            input.lines().map(|line| Some(line.split_at(3))).collect();
        let mut ip = 0;
        let result = loop {
            if ip >= program.len() {
                break acc;
            }
            if let Some((op, operand)) = program[ip] {
                program[ip] = None;
                let mov = match op {
                    "acc" => {
                        acc += operand.trim().parse::<isize>()?;
                        1
                    }
                    "jmp" => operand.trim().parse::<isize>()?,
                    "nop" => 1,
                    _ => unreachable!(),
                };
                ip = ((ip as isize) + mov) as usize;
            } else {
                break acc;
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::part_one;
    use std::fs;

    const EXAMPLE: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;

    #[test]
    fn part_one_solution_example() {
        let result = part_one::solution(&EXAMPLE).unwrap();
        assert_eq!(5, result);
    }

    #[test]
    fn part_one_solution() {
        let s = fs::read_to_string("resources/day8.txt").unwrap();
        let result = part_one::solution(&s).unwrap();
        assert_eq!(1134, result);
    }
}
