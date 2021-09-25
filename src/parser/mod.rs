use anyhow::{ Result, anyhow };
use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    combinator::*,
    error::ParseError,
    multi::{many0, many1},
    number::complete::*,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

mod string;
use string::*;

use crate::command::{Command, Identifier, Operation, Value};
use crate::errors::*;

fn int_to_value(input: &str) -> Result<Value> {
    Ok(Value::Integer(i64::from_str_radix(input, 10)?))
}

fn float_to_value(input: &str) -> Result<Value> {
    Ok(Value::Float(input.parse::<f64>()?))
}

fn decimal(input: &str) -> IResult<&str, Value> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        int_to_value,
    )(input)
}

fn float(input: &str) -> IResult<&str, Value> {
    map_res(
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
            ))), // Case two: 42e42 and 42.42e42
            recognize(tuple((
                decimal,
                opt(preceded(char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))), // Case three: 42. and 42.42
            recognize(tuple((decimal, char('.'), opt(decimal)))),
        )),
        float_to_value,
    )(input)
}

fn string(input: &str) -> IResult<&str, Value> {
    map_res(string::parse_string, |out: String| -> Result<Value> {
        Ok(Value::String(out))
    })(input)
}

fn identifier(input: &str) -> IResult<&str, Identifier> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |out: &str| -> Result<Identifier> { Ok(Identifier(out.into())) },
    )(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    map_res(
        alt((tag("enqueue"), tag("dequeue"))),
        |out: &str| -> Result<Operation> {
            match out {
                "enqueue" => Ok(Operation::Enqueue),
                "dequeue" => Ok(Operation::Dequeue),
                a => Err(anyhow!(SyntaxError::InvalidIdentifier(a.into())))
            }
        },
    )(input)
}

pub fn parse_command(input: &str) -> IResult<&str, (Operation, &str, Identifier, &str, Value)> {
    tuple((
        operation,
        multispace1,
        identifier,
        multispace1,
        alt((decimal, float, string)),
    ))(input)
}

#[test]
fn decimal_test() {
    assert_eq!(decimal("1"), Ok(("", Value::Integer(1))));
    assert_eq!(decimal("2"), Ok(("", Value::Integer(2))));
    assert_eq!(decimal("3"), Ok(("", Value::Integer(3))));
    assert_eq!(decimal("4"), Ok(("", Value::Integer(4))));
    assert_eq!(decimal("5"), Ok(("", Value::Integer(5))));
    assert_eq!(decimal("123456"), Ok(("", Value::Integer(123456))));
    assert!(decimal("a").is_err());
}

#[test]
fn float_test() {
    assert_eq!(float("1.0"), Ok(("", Value::Float(1.0))));
    assert_eq!(float("2.0"), Ok(("", Value::Float(2.0))));
    assert_eq!(float("3.9"), Ok(("", Value::Float(3.9))));
    assert_eq!(float("4.0"), Ok(("", Value::Float(4.0))));
    assert_eq!(float("5.0"), Ok(("", Value::Float(5.0))));
    assert_eq!(float("123456.1"), Ok(("", Value::Float(123456.1))));
    assert!(float("a").is_err());
}

#[test]
fn command_test() {
    assert_eq!(
        parse_command("enqueue omg 123"),
        Ok((
            "",
            (
                Operation::Enqueue,
                " ",
                Identifier("omg".into()),
                " ",
                Value::Integer(123)
            )
        ))
    );
}
