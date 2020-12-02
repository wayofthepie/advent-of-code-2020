use std::str::FromStr;

use nom::{
    bytes::complete::is_not, bytes::complete::tag, bytes::complete::take_till,
    character::complete::multispace0, combinator::map_res, error::ParseError, multi::many1,
    sequence::delimited, IResult,
};

pub struct Details {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

pub fn part_one(s: &str) -> IResult<&str, usize> {
    let (rem, details) = many1(ws(parse_line))(s)?;
    let correct_passwords = details.iter().fold(0, |correct, details| {
        let count = details.password.chars().fold(0, |count, ch| {
            if ch == details.letter {
                count + 1
            } else {
                count
            }
        });
        if count >= details.min && count <= details.max {
            correct + 1
        } else {
            correct
        }
    });
    Ok((rem, correct_passwords))
}

fn parse_line(line: &str) -> IResult<&str, Details> {
    let (rem, min) = parse_to_and_eat(line, '-')?;
    let (rem, max) = parse_to_and_eat(rem, ' ')?;
    let (rem, letter) = parse_to_and_eat(rem, ':')?;
    let (rem, password) = ws(is_not_newline())(rem)?;
    let details = Details {
        min,
        max,
        letter,
        password: password.to_owned(),
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

fn is_not_newline<'a, E: ParseError<&'a str>>(
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> {
    is_not("\r\n")
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::part_one;

    #[test]
    fn should_parse_without_error() {
        let s = fs::read_to_string("resources/day2.txt").unwrap();
        let result = part_one(&s).unwrap();
        assert_eq!(result.1, 396);
    }
}
