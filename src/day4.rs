use nom::{
    branch::alt, bytes::complete::is_not, bytes::complete::tag, character::complete::alpha1,
    combinator::opt, multi::many1, sequence::separated_pair, IResult,
};

pub mod part_one {
    use super::{parse, Passport};
    use nom::IResult;

    pub fn solution(input: &str) -> IResult<&str, usize> {
        let (rem, passports) = parse(input)?;
        let valid = passports
            .iter()
            .filter(|passport| {
                matches!(passport, Passport {
                    byr: Some(_),
                    iyr: Some(_),
                    eyr: Some(_),
                    hgt: Some(_),
                    hcl: Some(_),
                    ecl: Some(_),
                    pid: Some(_),
                    cid: _
                })
            })
            .count();
        Ok((rem, valid))
    }
}

#[derive(Debug, Default)]
struct Passport<'input> {
    byr: Option<&'input str>,
    iyr: Option<&'input str>,
    eyr: Option<&'input str>,
    hgt: Option<&'input str>,
    hcl: Option<&'input str>,
    ecl: Option<&'input str>,
    pid: Option<&'input str>,
    cid: Option<&'input str>,
}

fn parse(input: &str) -> IResult<&str, Vec<Passport>> {
    many1(parse_entry)(input)
}

fn parse_entry(input: &str) -> IResult<&str, Passport> {
    let mut passport = Passport::default();
    let (rem, fields) = many1(parse_field)(input)?;
    for (name, value) in fields {
        match name {
            "byr" => passport.byr = Some(value),
            "iyr" => passport.iyr = Some(value),
            "eyr" => passport.eyr = Some(value),
            "hgt" => passport.hgt = Some(value),
            "hcl" => passport.hcl = Some(value),
            "ecl" => passport.ecl = Some(value),
            "pid" => passport.pid = Some(value),
            "cid" => passport.cid = Some(value),
            _ => unreachable!(),
        }
    }
    let (rem, _) = opt(tag("\n"))(rem)?;
    println!("{:#?}", passport);
    Ok((rem, passport))
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
    use super::part_one;
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
}
