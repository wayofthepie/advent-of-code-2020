use std::fs;

use aoc2020::day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if let Some(day_id) = args.get(0) {
        let solution = match day_id.as_str() {
            "day1:part1" => {
                let s = fs::read_to_string("resources/day1.txt")?;
                day1::part_one_solution(&s, 2020)
            }
            "day1:part2" => {
                let s = fs::read_to_string("resources/day1.txt")?;
                day1::part_two_solution(&s, 2020)
            }
            _ => return Err("Unknown day:part ...".into()),
        };
        println!("Solution to {} is {}", day_id, solution);
        Ok(())
    } else {
        Err("You must pass a day and part in the format day:part to solve".into())
    }
}
