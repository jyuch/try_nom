use chrono::{DateTime, Local, TimeZone};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::{map, map_res};
use nom::{branch::permutation, IResult};
use std::num::ParseIntError;

#[allow(dead_code)]
fn datetime(input: &str) -> IResult<&str, DateTime<Local>> {
    let (input, year) = map_res(digit1, |it: &str| -> Result<i32, ParseIntError> {
        it.parse()
    })(input)?;
    let (input, _) = tag("年")(input)?;
    let (input, month) = map_res(digit1, |it: &str| -> Result<u32, ParseIntError> {
        it.parse()
    })(input)?;
    let (input, _) = tag("月")(input)?;
    let (input, day) = map_res(digit1, |it: &str| -> Result<u32, ParseIntError> {
        it.parse()
    })(input)?;
    let (input, _) = tag("日")(input)?;
    let (input, _) = space1(input)?;
    let (input, h) = map_res(digit1, |it: &str| -> Result<u32, ParseIntError> {
        it.parse()
    })(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, m) = map_res(digit1, |it: &str| -> Result<u32, ParseIntError> {
        it.parse()
    })(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, s) = map_res(digit1, |it: &str| -> Result<u32, ParseIntError> {
        it.parse()
    })(input)?;

    let d = Local.ymd(year, month, day).and_hms(h, m, s);
    Ok((input, d))
}

#[allow(dead_code)]
fn start_datetime(input: &str) -> IResult<&str, DateTime<Local>> {
    map(
        permutation((tag("開始"), space0, tag(":"), space0, datetime)),
        |(_, _, _, _, d): (&str, &str, &str, &str, DateTime<Local>)| -> DateTime<Local> { d },
    )(input)
}

#[test]
fn datetime_can_parse_robocopy_datetime_format() {
    let expected = Local.ymd(2022, 2, 12).and_hms(19, 58, 39);
    let (_, actual) = datetime("2022年2月12日 19:58:39").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn start_datetime_can_parse_robocopy_start_datetime() {
    let expected = Local.ymd(2022, 2, 12).and_hms(19, 58, 6);
    let (_, actual) = start_datetime("開始: 2022年2月12日 19:58:06").unwrap();
    assert_eq!(expected, actual);
}
