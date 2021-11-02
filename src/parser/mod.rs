use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    combinator::*,
    multi::{many0, many1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

mod string;

use crate::errors::*;
use crate::types::{Command, Identifier, Value};

fn int_to_value(input: &str) -> Result<Value> {
    Ok(i64::from_str_radix(input, 10)?.into())
}

fn decimal(input: &str) -> IResult<&str, Value> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        int_to_value,
    )(input)
}

fn float(input: &str) -> IResult<&str, Value> {
    map_res(nom::number::complete::float, |out: f32| -> Result<Value> {
        Ok(out.into())
    })(input)
}

fn string(input: &str) -> IResult<&str, Value> {
    map_res(string::parse_string, |out: String| -> Result<Value> {
        Ok(out.into())
    })(input)
}

fn null(input: &str) -> IResult<&str, Value> {
    value(Value::Null, tag("null"))(input)
}

fn identifier(input: &str) -> IResult<&str, Identifier> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |out: &str| -> Result<Identifier> { Ok(out.into()) },
    )(input)
}

fn val(input: &str) -> IResult<&str, Value> {
    alt((decimal, float, string, null))(input)
}

fn enqueue(input: &str) -> IResult<&str, Command> {
    map_res(
        tuple((tag("enqueue"), tag(" "), identifier, tag(" "), val)),
        |(_, _, id, _, val): (&str, &str, Identifier, &str, Value)| -> Result<Command> {
            Ok(Command::enqueue(id, val))
        },
    )(input)
}

fn dequeue(input: &str) -> IResult<&str, Command> {
    map_res(
        tuple((tag("dequeue"), multispace1, identifier)),
        |(_, _, id): (&str, &str, Identifier)| -> Result<Command> { Ok(Command::Dequeue(id)) },
    )(input)
}

fn length(input: &str) -> IResult<&str, Command> {
    map_res(
        tuple((tag("length"), multispace1, identifier)),
        |(_, _, id): (&str, &str, Identifier)| -> Result<Command> { Ok(Command::Length(id)) },
    )(input)
}

fn peek(input: &str) -> IResult<&str, Command> {
    map_res(
        tuple((tag("peek"), multispace1, identifier)),
        |(_, _, id): (&str, &str, Identifier)| -> Result<Command> { Ok(Command::Peek(id)) },
    )(input)
}

fn assert(input: &str) -> IResult<&str, Command> {
    let inner = delimited(tag("("), expr, tag(")"));
    let with_spaces = delimited(multispace1, inner, multispace1);

    map_res(
        tuple((tag("assert"), with_spaces, val)),
        |(_, cmd, val): (&str, Command, Value)| -> Result<Command> {
            Ok(Command::Assert(Box::new(cmd), val))
        },
    )(input)
}

fn assert_error(input: &str) -> IResult<&str, Command> {
    let inner = delimited(tag("("), expr, tag(")"));
    let with_spaces = delimited(multispace1, inner, multispace1);

    map_res(
        tuple((tag("assert error"), with_spaces)),
        |(_, cmd): (&str, Command)| -> Result<Command> { Ok(Command::AssertError(Box::new(cmd))) },
    )(input)
}

fn comment(input: &str) -> IResult<&str, Command> {
    value(Command::Noop, pair(char('#'), is_not("\r\n")))(input)
}

#[tracing::instrument]
pub fn expr(input: &str) -> IResult<&str, Command> {
    complete(alt((
        comment,
        enqueue,
        dequeue,
        length,
        peek,
        assert,
        assert_error,
    )))(input)
}

#[tracing::instrument]
pub fn program(input: &str) -> IResult<&str, Vec<Command>> {
    many1(terminated(expr, opt(line_ending)))(input)
}

pub fn parse(input: &str) -> Result<Vec<Command>> {
    let (_, prg) = program(input).map_err(|_| SyntaxError::ParseError(input.to_string()))?;

    Ok(prg)
}

#[test]
fn decimal_test() {
    assert_eq!(decimal("1"), Ok(("", 1.into())));
    assert_eq!(decimal("2"), Ok(("", 2.into())));
    assert_eq!(decimal("3"), Ok(("", 3.into())));
    assert_eq!(decimal("4"), Ok(("", 4.into())));
    assert_eq!(decimal("5"), Ok(("", 5.into())));
    assert_eq!(decimal("123456"), Ok(("", Value::Integer(123456))));
    assert!(decimal("a").is_err());
}

#[test]
fn float_test() {
    assert_eq!(float("1.0"), Ok(("", (1.0).into())));
    assert_eq!(float("2.0"), Ok(("", (2.0).into())));
    assert_eq!(float("4.0"), Ok(("", (4.0).into())));
    assert_eq!(float("5.0"), Ok(("", (5.0).into())));
    assert!(float("a").is_err());
}

#[test]
fn null_test() {
    assert_eq!(null("null"), Ok(("", Value::Null)));
    assert_eq!(
        expr("enqueue a null"),
        Ok(("", Command::enqueue("a", Value::Null)))
    );
}

#[test]
fn expr_test() {
    assert_eq!(
        expr("enqueue omg 123"),
        Ok(("", Command::enqueue("omg", 123)))
    );
    assert_eq!(expr("dequeue omg"), Ok(("", Command::dequeue("omg"))));
    assert_eq!(expr("length omg"), Ok(("", Command::length("omg"))));
    assert_eq!(expr("peek omg"), Ok(("", Command::peek("omg"))));
    assert_eq!(
        expr("assert (peek omg) 1"),
        Ok((
            "",
            Command::Assert(Box::new(Command::peek("omg")), 1.into())
        ))
    );
}

#[test]
fn program_test() -> Result<()> {
    assert_eq!(
        parse("enqueue omg 123\r\ndequeue omg\r\nlength omg")?,
        vec![
            Command::enqueue("omg", 123),
            Command::dequeue("omg"),
            Command::length("omg")
        ]
    );

    Ok(())
}
