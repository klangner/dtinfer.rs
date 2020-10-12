use nom::error::{ErrorKind, ParseError};
use nom;


#[derive(Debug, PartialEq)]
pub enum DateTimeError<I> {
  NotMonth,
  NotDay,
  NotTimePart,
  NotSeparator,
  Nom(I, ErrorKind),
}

impl<I> ParseError<I> for DateTimeError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        DateTimeError::Nom(input, kind)
    }
    
    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> From<(&'a [u8], ErrorKind)> for DateTimeError<&'a [u8]> {
    fn from((i, ek): (&'a [u8], ErrorKind)) -> Self {
        DateTimeError::Nom(i, ek)
    }
}