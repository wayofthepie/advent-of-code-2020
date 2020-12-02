use nom::{
    bytes::complete::{is_not, tag, take_till},
    character::complete::multispace0,
    combinator::map_res,
    sequence::delimited,
    IResult,
};
use std::str::FromStr;

struct Details<'details> {
    first: usize,
    second: usize,
    letter: char,
    password: &'details str,
}

pub mod part_one {
    use super::{parse_line, ws};
    use nom::{multi::many1, IResult};

    pub fn solution(s: &str) -> IResult<&str, usize> {
        let (rem, details) = many1(ws(parse_line))(s)?;
        let correct_passwords = details
            .iter()
            .filter(|details| {
                let count = details
                    .password
                    .chars()
                    .filter(|&c| c == details.letter)
                    .count();
                (details.first..=details.second).contains(&count)
            })
            .count();
        Ok((rem, correct_passwords))
    }
}

pub mod part_two {
    use super::{parse_line, ws, Details};
    use nom::{multi::many1, IResult};

    pub fn solution(s: &str) -> IResult<&str, usize> {
        let (rem, details) = many1(ws(parse_line))(s)?;
        let correct_passwords = details
            .into_iter()
            .filter(
                |Details {
                     first,
                     second,
                     letter,
                     password,
                 }| {
                    let tuple = (
                        password.chars().nth(first - 1),
                        password.chars().nth(second - 1),
                    );
                    matches!(tuple, (f, s) if (f == Some(*letter)) != (s == Some(*letter)))
                },
            )
            .count();
        Ok((rem, correct_passwords))
    }
}

fn parse_line(line: &str) -> IResult<&str, Details> {
    let (rem, first) = parse_to_and_eat(line, '-')?;
    let (rem, second) = parse_to_and_eat(rem, ' ')?;
    let (rem, letter) = parse_to_and_eat(rem, ':')?;
    let (rem, password) = ws(is_not_newline())(rem)?;
    let details = Details {
        first,
        second,
        letter,
        password,
    };
    Ok((rem, details))
}

fn parse_to_and_eat<F: FromStr>(s: &str, till: char) -> IResult<&str, F> {
    let (rem, result) = map_res(take_till(move |c| c == till), str_to::<F>)(s)?;
    let mut slice = [0; 1];
    let s = till.encode_utf8(&mut slice);
    let (rem, _) = tag(&*s)(rem)?;
    Ok((rem, result))
}

fn str_to<F: FromStr>(s: &str) -> Result<F, F::Err> {
    s.parse::<F>()
}

fn is_not_newline<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    is_not("\r\n")
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::{part_one, part_two};

    #[test]
    fn part_one_should_parse_without_error() {
        let s = fs::read_to_string("resources/day2.txt").unwrap();
        let (rem, result) = part_one::solution(&s).unwrap();
        assert_eq!(result, 396);
        assert!(rem.is_empty())
    }

    #[test]
    fn part_two_should_parse_without_error() {
        let s = fs::read_to_string("resources/day2.txt").unwrap();
        let (rem, result) = part_two::solution(&s).unwrap();
        assert_eq!(result, 428);
        assert!(rem.is_empty())
    }
}
