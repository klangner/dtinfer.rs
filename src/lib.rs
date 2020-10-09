//! # Infer Datetime format from the given string
//!


pub fn infer_best(sample_date: &str) -> &str {
    "%Y-%m-%d %H:%M:%S"
}

enum DateTimePart {
    Year, Month, Day,
    Hour, Minute, Second,
}

struct Pattern {
    parts: Vec<DateTimePart>,
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

use super::infer_best;

    #[test]
    fn iso_8601_date_time() {
        let pattern = infer_best("2020-10-09 15:24:50");
        assert_eq!(pattern, "%Y-%m-%d %H:%M:%S");
    }

    #[test]
    fn iso_8601_date_only() {
        let pattern = infer_best("2020-10-09");
        assert_eq!(pattern, "%Y-%m-%d");
    }

}
