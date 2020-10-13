//! # Infer Datetime format from the given string
//!
//! 

use std::str;
use nom::{
    bytes::complete::{take_while, take_while_m_n},
    character::{complete::char, is_digit},
    combinator::opt,
    sequence::tuple, 
    IResult,
    Err::Error,
};

use crate::error::DateTimeError;


#[derive(Clone)]
enum DateTimePart {
    Year, Month, Day,
    Hour, Minute, Second, Timezone,
    Separator(String),
}

impl DateTimePart {
    fn as_str(&self) -> &str {
        match self {
            DateTimePart::Year => "%Y",
            DateTimePart::Month => "%m",
            DateTimePart::Day => "%d",
            DateTimePart::Hour => "%H",
            DateTimePart::Minute => "%M",
            DateTimePart::Second => "%S",
            DateTimePart::Timezone => "%z",
            DateTimePart::Separator(s) => s,
        }
    }
}

#[derive(Clone)]
pub struct Pattern {
    parts: Vec<DateTimePart>
}

impl Pattern {
    fn new() -> Pattern {
        Pattern{ parts: vec![] }
    }

    fn from_parts(parts: Vec<DateTimePart>) -> Pattern {
        Pattern{ parts }
    }

    pub fn as_str(&self) -> String {
        self.parts.iter()
            .map(|t| t.as_str())
            .collect::<Vec<&str>>()
            .join("")
    }
}

type CustomResult<I> = IResult<I, DateTimePart, DateTimeError<I>>;

pub fn parse_sample_date(input: &[u8]) -> IResult<&[u8], Pattern, DateTimeError<&[u8]>> {
    let (i1, date_pattern) = parse_date(input)?;
    let (i2, sep) = separator(i1).unwrap_or((i1, DateTimePart::Separator("".to_owned())));
    let (i3, time_pattern) = parse_time(i2).unwrap_or((i1, Pattern::new()));

    let mut parts = vec![];
    parts.extend_from_slice(&date_pattern.parts[..]);
    parts.extend_from_slice(&vec![sep]);
    parts.extend_from_slice(&time_pattern.parts[..]);
    Ok((i3, Pattern::from_parts(parts)))
}

/// Year is mandatory but Month and day is optional
fn parse_date(input: &[u8]) -> IResult<&[u8], Pattern, DateTimeError<&[u8]>> {
    let mut pattern = Pattern::new();
    let parsers = tuple((year, opt(separator), opt(month), opt(separator), opt(day)));
    let (i, (y, s1, m, s2, d)) = parsers(input)?;

    pattern.parts.push(y);

    if let Some(p) = s1 {
        pattern.parts.push(p);
    } 
    
    if let Some(p) = m {
        pattern.parts.push(p);
    } 
    
    if let Some(p) = s2 {
        pattern.parts.push(p);
    } 
    
    if let Some(p) = d {
        pattern.parts.push(p);
    } 

    Ok((i, pattern))
}

fn year(i: &[u8]) -> CustomResult<&[u8]> {
    let (i, _) = take_while_m_n(4, 4, is_digit)(i)?;
    Ok((i, DateTimePart::Year))
}

fn month(i: &[u8]) -> CustomResult<&[u8]> {
    let (i, vs) = take_while_m_n(1, 2, is_digit)(i)?;
    let m: u32 = str::from_utf8(vs).expect("Non unicode character")
        .parse::<u32>().expect("Can't convert to u32");
    if m < 13 { Ok((i, DateTimePart::Month)) }
    else { Err(Error(DateTimeError::NotMonth)) }
}

fn day(i: &[u8]) -> CustomResult<&[u8]> {
    let (i, vs) = take_while_m_n(1, 2, is_digit)(i)?;
    let m: u32 = str::from_utf8(vs).expect("Non unicode character")
        .parse::<u32>().expect("Can't convert to u32");
    if m < 32 { Ok((i, DateTimePart::Day)) }
    else { Err(Error(DateTimeError::NotDay)) }
}

/// Hour is mandatory but rest is optional
fn parse_time(input: &[u8]) -> IResult<&[u8], Pattern, DateTimeError<&[u8]>> {
    let mut pattern = Pattern::new();
    let parsers = tuple((time_part, opt(separator), opt(time_part), opt(separator), opt(time_part), opt(timezone)));
    let (i, (h, s1, m, s2, s, z)) = parsers(input)?;

    if h > 24 { return Err(Error(DateTimeError::NotTimePart)) }

    pattern.parts.push(DateTimePart::Hour);

    if let Some(p) = s1 {
        pattern.parts.push(p);
    } 
    
    if let Some(_) = m {
        pattern.parts.push(DateTimePart::Minute);
    } 
    
    if let Some(p) = s2 {
        pattern.parts.push(p);
    } 
    
    if let Some(_) = s {
        pattern.parts.push(DateTimePart::Second);
    } 

    if let Some(_) = z {
        pattern.parts.push(DateTimePart::Timezone);
    } 

    Ok((i, pattern))
}

fn time_part(i: &[u8]) -> IResult<&[u8], u32, DateTimeError<&[u8]>> {
    let (i, vs) = take_while_m_n(2, 2, is_digit)(i)?;
    let v: u32 = str::from_utf8(vs).expect("Non unicode character")
        .parse::<u32>().expect("Can't convert to u32");
    Ok((i, v)) 
}

fn timezone(input: &[u8]) -> CustomResult<&[u8]> {
    let (i, _) = char('+')(input)?;
    let (i, _) = take_while_m_n(2, 2, is_digit)(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = take_while_m_n(2, 2, is_digit)(i)?;
    Ok((i, DateTimePart::Timezone)) 
}

fn separator(i: &[u8]) -> CustomResult<&[u8]> {
    let (i, vs) = take_while(not_digit)(i)?;
    let text = str::from_utf8(vs).expect("Non unicode character");
    if text.len() > 0 {Ok((i, DateTimePart::Separator(text.to_owned())))}
    else {Err(Error(DateTimeError::NotSeparator))}
}

fn not_digit(chr: u8) -> bool {
    !is_digit(chr)
}

