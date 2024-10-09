use jiff::{civil, fmt::temporal};

static DATE_PARSER: temporal::DateTimeParser = temporal::DateTimeParser::new();

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SegType {
    Number,
    String,
    Date,
}
impl SegType {
    fn match_string(input: &str) -> Option<MatchValue> {
        Some(MatchValue::from_str(input))
    }
    fn match_number(input: &str) -> Option<MatchValue> {
        input
            .parse::<f64>()
            .ok()
            .map(|v| MatchValue::from_number(v))
    }
    fn match_date(input: &str) -> Option<MatchValue> {
        DATE_PARSER
            .parse_date(input)
            .ok()
            .map(|v| MatchValue::from_date(v))
    }
    fn match_segment(&self, input: &str) -> Option<MatchValue> {
        match self {
            SegType::String => Self::match_string(input),
            SegType::Number => Self::match_number(input),
            SegType::Date => Self::match_date(input),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub name: String,
    seg_type: SegType,
}
impl Var {
    pub fn new(name: String, seg_type: SegType) -> Self {
        Self { name, seg_type }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Static(String),
    Var(Var),
    Terminus,
}

impl Segment {
    pub fn match_segment(&self, input: &str) -> MatchResult {
        match self {
            Segment::Static(s) => {
                if input == s.as_str() {
                    MatchResult::new_unnamed(MatchValue::from_str(input))
                } else {
                    MatchResult::NotMatched
                }
            }
            Segment::Terminus => {
                if input.is_empty() {
                    MatchResult::terminus()
                } else {
                    MatchResult::NotMatched
                }
            }
            Segment::Var(v) => {
                if let Some(parsed) = v.seg_type.match_segment(input) {
                    MatchResult::new_named(parsed, v.name.clone())
                } else {
                    MatchResult::NotMatched
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    NotMatched,
    Matched {
        value: MatchValue,
        name: Option<String>,
    },
}

impl MatchResult {
    fn new_named(value: MatchValue, name: String) -> Self {
        Self::Matched {
            value,
            name: Some(name),
        }
    }
    fn new_unnamed(value: MatchValue) -> Self {
        Self::Matched { value, name: None }
    }
    fn terminus() -> Self {
        Self::Matched {
            value: MatchValue::Terminus,
            name: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchValue {
    String(String),
    Number(f64),
    Date(civil::Date),
    Terminus,
}

impl MatchValue {
    fn from_str(input: &str) -> Self {
        Self::String(input.to_string())
    }
    fn from_number(input: f64) -> Self {
        Self::Number(input)
    }
    fn from_date(input: civil::Date) -> Self {
        Self::Date(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::errors::MatchError;

    #[test]
    fn seg_type_match_string() {
        let result = SegType::match_string("hello");
        assert_eq!(result, Some(MatchValue::String(String::from("hello"))));
    }
    #[test]
    fn seg_type_match_number_ok() {
        let result = SegType::match_number("123.45");
        assert_eq!(result, Some(MatchValue::Number(123.45)));
    }
    #[test]
    fn seg_type_match_number_err() {
        let result = SegType::match_number("123.45.67");
        assert!(result.is_none());
    }
    #[test]
    fn seg_type_match_date_ok() {
        let result = SegType::match_date("2021-01-01");
        assert_eq!(result, Some(MatchValue::Date(civil::date(2021, 1, 1))));
    }
    #[test]
    fn seg_type_match_date_err() {
        let result = SegType::match_date("2021-01-01-01");
        assert!(result.is_none());
    }
    #[test]
    fn segment_static_match_ok() {
        let segment = Segment::Static("hello".to_string());
        let result = segment.match_segment("hello");
        assert_eq!(
            result,
            MatchResult::Matched {
                value: MatchValue::String("hello".to_string()),
                name: None
            }
        )
    }
    #[test]
    fn segment_static_match_err() {
        let segment = Segment::Static("hello".to_string());
        let result = segment.match_segment("world");
        assert_eq!(result, MatchResult::NotMatched);
    }
    #[test]
    fn segment_terminus_match_ok() {
        let segment = Segment::Terminus;
        let result = segment.match_segment("");
        assert_eq!(
            result,
            MatchResult::Matched {
                value: MatchValue::Terminus,
                name: None
            }
        );
    }
    #[test]
    fn segment_terminus_match_err() {
        let segment = Segment::Terminus;
        let result = segment.match_segment("world");
        assert_eq!(result, MatchResult::NotMatched)
    }
    #[test]
    fn segment_var_match_ok() {
        let segment = Segment::Var(Var::new("num".to_string(), SegType::Number));
        let result = segment.match_segment("123.45");
        assert_eq!(
            result,
            MatchResult::Matched {
                value: MatchValue::Number(123.45),
                name: Some("num".to_string())
            }
        );
    }
    #[test]
    fn segment_var_match_err() {
        let segment = Segment::Var(Var::new("num".to_string(), SegType::Number));
        let result = segment.match_segment("world");
        assert_eq!(result, MatchResult::NotMatched)
    }
}
