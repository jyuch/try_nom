use nom::branch::{alt, permutation};
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res, opt};
use nom::IResult;
use std::num::ParseIntError;

#[allow(dead_code)]
fn plus_sign(input: &str) -> IResult<&str, i32> {
    map(char('+'), |_: char| -> i32 { 1 })(input)
}

#[test]
fn plus_sign_test() {
    let expected = 1;
    let (_, actual) = plus_sign("+").unwrap();
    assert_eq!(expected, actual);
}

#[allow(dead_code)]
fn minus_sign(input: &str) -> IResult<&str, i32> {
    map(char('-'), |_: char| -> i32 { -1 })(input)
}

#[test]
fn minus_sign_test() {
    let expected = -1;
    let (_, actual) = minus_sign("-").unwrap();
    assert_eq!(expected, actual);
}

#[allow(dead_code)]
fn sign(input: &str) -> IResult<&str, i32> {
    map(opt(alt((plus_sign, minus_sign))), |s: Option<i32>| -> i32 {
        s.unwrap_or_else(|| 1)
    })(input)
}

#[test]
fn sign_can_parse_empty() {
    let expected = 1;
    let (_, actual) = sign("").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn sign_can_parse_plus_sign() {
    let expected = 1;
    let (_, actual) = sign("+").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn sign_can_parse_minus_sign() {
    let expected = -1;
    let (_, actual) = sign("-").unwrap();
    assert_eq!(expected, actual);
}

#[allow(dead_code)]
fn unsigned_integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |i: &str| -> Result<i32, ParseIntError> {
        i.parse::<i32>()
    })(input)
}

#[test]
fn unsigned_integer_can_parse() {
    let expected = 12345;
    let (_, actual) = unsigned_integer("12345").unwrap();
    assert_eq!(expected, actual);
}

#[allow(dead_code)]
fn signed_integer(input: &str) -> IResult<&str, i32> {
    map(
        permutation((sign, unsigned_integer)),
        |(s, i): (i32, i32)| -> i32 { s * i },
    )(input)
}

#[test]
fn signed_integer_can_parse_plus_value() {
    let expected = 12345;
    let (_, actual) = signed_integer("+12345").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn signed_integer_can_parse_minus_value() {
    let expected = -12345;
    let (_, actual) = signed_integer("-12345").unwrap();
    assert_eq!(expected, actual);
}
