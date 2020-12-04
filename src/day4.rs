use std::collections::HashMap;

use nom::{
    branch::alt, bytes::complete::is_not, bytes::complete::tag, character::complete::alpha1,
    combinator::opt, multi::many1, sequence::separated_pair, IResult,
};

pub mod part_one {
    use super::parse;
    use nom::IResult;

    pub fn solution<'input>(input: &'input str) -> IResult<&'input str, usize> {
        let (rem, passports) = parse(input)?;
        let valid = passports
            .iter()
            .filter(|&details| {
                ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                    .iter()
                    .all(|key| details.contains_key(key))
            })
            .count();
        Ok((rem, valid))
    }
}

pub mod part_two {
    use super::parse;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, alphanumeric1, digit1},
        combinator::map_res,
        IResult,
    };
    use std::{collections::HashMap, str::FromStr};

    pub fn solution<'input>(input: &'input str) -> IResult<&'input str, usize> {
        let (rem, passports) = parse(input)?;
        let valid = passports.into_iter().filter(is_valid_passport).count();
        Ok((rem, valid))
    }

    fn is_valid_passport(details: &HashMap<&str, &str>) -> bool {
        validate_year_str(details.get("byr"), 1920, 2002)
            && validate_year_str(details.get("iyr"), 2010, 2020)
            && validate_year_str(details.get("eyr"), 2020, 2030)
            && map_with(details, "hgt", |value| hgt(value).unwrap_or(("", false)).1)
            && map_with(details, "hcl", |value| hcl(value).unwrap_or(("", false)).1)
            && map_with(details, "ecl", |value| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value)
            })
            && details.get("pid").map(|value| pid(value)).unwrap_or(false)
    }

    fn map_with(details: &HashMap<&str, &str>, key: &str, f: impl FnMut(&&str) -> bool) -> bool {
        details.get(key).map(f).unwrap_or(false)
    }

    fn hgt(value: &str) -> IResult<&str, bool> {
        let (rem, digits) = map_res(digit1, str_to::<u16>)(value)?;
        let (rem, units) = alpha1(rem)?;
        let validated = match units {
            "cm" => (150..=193).contains(&digits),
            "in" => (59..=76).contains(&digits),
            _ => false,
        };
        Ok((rem, validated))
    }

    fn hcl(value: &str) -> IResult<&str, bool> {
        let (rem, _) = tag("#")(value)?;
        let (rem, s) = alphanumeric1(rem)?;
        Ok((rem, rem.is_empty() && s.len() == 6))
    }

    fn pid(value: &str) -> bool {
        if value.len() == 9 {
            matches!(str_to::<usize>(value), Ok(_))
        } else {
            false
        }
    }

    fn validate_year_str(value: Option<&&str>, min: u16, max: u16) -> bool {
        value.map_or(false, |value| {
            value
                .parse::<u16>()
                .ok()
                .map(|year| (min..=max).contains(&year))
                .unwrap_or(false)
        })
    }

    fn str_to<F: FromStr>(s: &str) -> Result<F, F::Err> {
        s.parse::<F>()
    }
}

fn parse(input: &str) -> IResult<&str, Vec<HashMap<&str, &str>>> {
    many1(parse_entry)(input)
}

fn parse_entry(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    let (rem, fields) = many1(parse_field)(input)?;
    let details: HashMap<_, _> = fields.into_iter().collect();
    let (rem, _) = opt(tag("\n"))(rem)?;
    Ok((rem, details))
}

fn parse_field(input: &str) -> IResult<&str, (&str, &str)> {
    let (rem, (name, value)) = separated_pair(parse_name(), tag(":"), parse_value())(input)?;
    let (rem, _) = opt(alt((tag(" "), tag("\n"))))(rem)?;
    Ok((rem, (name, value)))
}

fn parse_name() -> impl FnMut(&str) -> IResult<&str, &str> {
    |input| alpha1(input)
}

fn parse_value() -> impl FnMut(&str) -> IResult<&str, &str> {
    |input| is_not("\n ")(input)
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};
    use std::fs;

    const EXAMPLE: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn part_one_solution_with_example() {
        let result = part_one::solution(&EXAMPLE);
        assert_eq!(("", 2), result.unwrap());
    }

    #[test]
    fn part_one_solution() {
        let s = fs::read_to_string("resources/day4.txt").unwrap();
        let result = part_one::solution(&s);
        assert_eq!(("", 208), result.unwrap());
    }

    const INVALID_PASSPORT: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

    #[test]
    fn part_two_should_count_no_valid_passports() {
        let result = part_two::solution(&INVALID_PASSPORT);
        assert_eq!(("", 0), result.unwrap());
    }

    const VALID_PASSPORTS: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

    #[test]
    fn part_two_should_count_valid_passports() {
        let result = part_two::solution(&VALID_PASSPORTS);
        assert_eq!(("", 4), result.unwrap());
    }

    #[test]
    fn part_two_solution() {
        let s = fs::read_to_string("resources/day4.txt").unwrap();
        let result = part_two::solution(&s);
        assert_eq!(("", 167), result.unwrap());
    }
}
