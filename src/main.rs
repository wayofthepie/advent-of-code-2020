use std::fs;

use aoc2020::{day1, day3};
use aoc2020::{day2, day4};

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
            "day2:part1" => {
                let s = fs::read_to_string("resources/day2.txt")?;
                let (_, num) = day2::part_one::solution(&s).map_err(|e| e.to_string())?;
                num
            }
            "day2:part2" => {
                let s = fs::read_to_string("resources/day2.txt")?;
                let (_, num) = day2::part_two::solution(&s).map_err(|e| e.to_string())?;
                num
            }
            "day3:part1" => {
                let s = fs::read_to_string("resources/day3.txt")?;
                day3::part_one::solution(&s)
            }
            "day3:part2" => {
                let s = fs::read_to_string("resources/day3.txt")?;
                day3::part_two::solution(&s)
            }
            "day4:part1" => {
                let s = fs::read_to_string("resources/day4.txt")?;
                let (_, valid) = day4::part_one::solution(&s).map_err(|e| e.to_string())?;
                valid
            }
            "day4:part3" => {
                let s = fs::read_to_string("resources/day4.txt")?;
                let (_, valid) = day4::part_two::solution(&s).map_err(|e| e.to_string())?;
                valid
            }
            _ => return Err("Unknown day:part ...".into()),
        };
        println!("Solution to {} is {}", day_id, solution);
        Ok(())
    } else {
        Err("You must pass a day and part in the format day:part to solve".into())
    }
}
