use chrono::{DateTime, Local, ParseResult, TimeZone};
use nom::{
    IResult,
    branch::permutation,
};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::{map, map_res};

#[allow(dead_code)]
fn to_datetime(input: &str) -> ParseResult<DateTime<Local>> {
    Local.datetime_from_str(input, "%Y年%m月%d日 %H:%M:%S")
}

#[allow(dead_code)]
fn datetime(input: &str) -> IResult<&str, DateTime<Local>> {
    map_res(
        permutation((
            digit1,
            tag("年"),
            digit1,
            tag("月"),
            digit1,
            tag("日"),
            space1,
            digit1,
            tag(":"),
            digit1,
            tag(":"),
            digit1)
        ),
        |(s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12): (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str)| -> ParseResult<DateTime<Local>> {
            let s = format!("{}{}{}{}{}{}{}{}{}{}{}{}", s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12);
            to_datetime(s.as_str())
        },
    )(input)
}

#[allow(dead_code)]
fn start_datetime(input: &str) -> IResult<&str, DateTime<Local>> {
    map(
        permutation((tag("開始"), space0, tag(":"), space0, datetime)),
        |(_, _, _, _, d): (&str, &str, &str, &str, DateTime<Local>)| -> DateTime<Local>  {
            d
        })(input)
}

#[test]
fn to_datetime_can_parse_robocopy_datetime_format() {
    let expected = Local.ymd(2022, 2, 12).and_hms(19, 58, 39);
    let actual = to_datetime("2022年2月12日 19:58:39").unwrap();
    assert_eq!(expected, actual);
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
