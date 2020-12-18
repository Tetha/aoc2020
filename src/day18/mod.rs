use std::num::ParseIntError;
use std::str::FromStr;

use nom::character::complete::char;
use nom::character::complete::{digit1 as digit, space0 as space};
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::{IResult};

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut sum = 0;
    for l in input.lines() {
        let expr = expression(l).unwrap();
        assert_eq!("", expr.0);
        sum += eval(expr.1);
    }
    println!("Result is {}", sum);
    Ok(())
}
#[derive(Debug,Clone, PartialEq)]
enum Expression {
    Number(i32),
    Add(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
}

fn eval(i: Expression) -> i64 {
    match i {
        Expression::Number(x) => x as i64,
        Expression::Add(e1, e2) => {
            eval(*e1) + eval(*e2)
        }
        Expression::Mult(e1, e2) => {
            eval(*e1) * eval(*e2)
        }
    }
}
fn lift_to_expression(input: &str) -> Result<Expression, ParseIntError> {
    input.parse::<i32>().map(|i| Expression::Number(i))
}
fn number(i: &str) -> IResult<&str, Expression> {
    nom::combinator::map_res(
        delimited(space, digit, space), 
        lift_to_expression,
    )(i)
}

fn parens(i: &str) -> IResult<&str, Expression> {
    delimited(space, delimited(tag("("),expression, tag(")")), space)(i)
}

fn factor(i: &str) -> IResult<&str, Expression> {
    nom::branch::alt((
        number,
        parens,
    ))(i)
}

fn expression(i: &str) -> IResult<&str, Expression> {
  let (i, init) = factor(i)?;

  nom::multi::fold_many0(
    nom::sequence::pair(nom::branch::alt((char('*'), char('+'))), factor),
    init,
    |acc, (op, val): (char, Expression)| {
      if op == '*' {
        Expression::Mult(Box::new(acc), Box::new(val))
      } else {
        Expression::Add(Box::new(acc), Box::new(val))
      }
    },
  )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stuff() {
        assert_eq!(Expression::Number(20), number("20").unwrap().1);
    }

    #[test]
    fn test_parse_sample() {
        println!("{:?}", expression("2 + 2 * 2"));
    }

    #[test]
    fn test_parse_sample_2() {
        println!("{:?}", expression("(2 + 2) * 2"));
    }

    #[test]
    fn test_example_1() {
        let input = "2 * 3 + (4 * 5)";
        let result = expression(input).unwrap();
        assert_eq!("", result.0);
        assert_eq!(26, eval(result.1));
    }
}