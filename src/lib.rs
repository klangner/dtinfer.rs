//! # Infer Datetime format from the given string
//!
//! Example
//! ```
//! use dtinfer;
//! use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
//! 
//! let sample = "1987-05-23T12:30";
//! let pattern = dtinfer::infer_best(sample).unwrap();
//! let parsed = NaiveDateTime::parse_from_str(sample, &pattern).unwrap();
//! let expected = NaiveDateTime::new(NaiveDate::from_ymd(1987, 5, 23), NaiveTime::from_hms(12, 30, 0));
//! assert_eq!(parsed, expected);
//! ```

mod error;
mod parser;

use parser::{parse_sample_date};



/// Try to infer date time format based on sample date
/// Example: 
/// let pattern = infer_best("2020-02-03 12:34:45");
/// assert_eq!(pattern, Some("%Y-%m-%d".to_owned()));
/// 
pub fn infer_best(sample_date: &str) -> Option<String> {
    match parse_sample_date(sample_date.as_bytes()) {
        Ok((_, pattern)) => Some(pattern.as_str()),
        Err(_) => None,
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

use super::infer_best;

    #[test]
    fn year() {
        let pattern = infer_best("2020").unwrap();
        assert_eq!(pattern, "%Y");
    }

    #[test]
    fn year_month() {
        let pattern = infer_best("2020-10").unwrap();
        assert_eq!(pattern, "%Y-%m");
    }

    #[test]
    fn year_month_day() {
        let pattern = infer_best("2020/10/23").unwrap();
        assert_eq!(pattern, "%Y/%m/%d");
    }

    #[test]
    fn iso_date_hour() {
        let pattern = infer_best("2020-10-09T15").unwrap();
        assert_eq!(pattern, "%Y-%m-%dT%H");
    }

    #[test]
    fn iso_8601_date_time() {
        let pattern = infer_best("2020-10-09 15:24:50").unwrap();
        assert_eq!(pattern, "%Y-%m-%d %H:%M:%S");
    }

    #[test]
    fn invalid_format() {
        let pattern = infer_best("invalid");
        assert_eq!(pattern, None);
    }

    #[test]
    fn iso_8601_date_only() {
        let pattern = infer_best("2020-10-09").unwrap();
        assert_eq!(pattern, "%Y-%m-%d");
    }

    #[test]
    fn iso_date_block() {
        let pattern = infer_best("20201009").unwrap();
        assert_eq!(pattern, "%Y%m%d");
    }

    #[test]
    fn time_zone() {
        let pattern = infer_best("2020-10-09 15:24:50+00:00").unwrap();
        assert_eq!(pattern, "%Y-%m-%d %H:%M:%S%z");
    }

}