use crate::errors::MatchError;
use jiff::{civil, fmt::temporal};

static DATE_PARSER: temporal::DateTimeParser = temporal::DateTimeParser::new();

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SegType {
    Number,
    String,
    Date,
}
impl SegType {
    fn match_string(input: &str) -> Result<MatchValue, MatchError> {
        Ok(MatchValue::from_str(input))
    }
    fn match_number(input: &str) -> Result<MatchValue, MatchError> {
        let num = input
            .parse::<f64>()
            .map_err(|_| MatchError::MatchError("number".to_string(), input.to_string()))?;
        Ok(MatchValue::from_number(num))
    }
    fn match_date(input: &str) -> Result<MatchValue, MatchError> {
        let parsed = DATE_PARSER
            .parse_date(input)
            .map_err(|_| MatchError::MatchError("date".to_string(), input.to_string()))?;
        Ok(MatchValue::from_date(parsed))
    }
    fn match_segment(&self, input: &str) -> Result<MatchValue, MatchError> {
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
    pub fn match_segment(&self, input: &str) -> Result<MatchResult, MatchError> {
        match self {
            Segment::Static(s) => {
                if input == s.as_str() {
                    Ok(MatchResult::new_unnamed(MatchValue::from_str(input)))
                } else {
                    Err(MatchError::MatchError(s.clone(), input.to_string()))
                }
            }
            Segment::Terminus => {
                if input.is_empty() {
                    Ok(MatchResult::terminus())
                } else {
                    Err(MatchError::MatchError(
                        "<Terminus>".to_string(),
                        input.to_string(),
                    ))
                }
            }
            Segment::Var(v) => {
                let parsed = v
                    .seg_type
                    .match_segment(input)
                    .map_err(|e| e.with_name(v.name.clone()))?;
                Ok(MatchResult::new_named(parsed, v.name.clone()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchResult {
    pub value: MatchValue,
    pub name: Option<String>,
}

impl MatchResult {
    fn new_named(value: MatchValue, name: String) -> Self {
        Self {
            value,
            name: Some(name),
        }
    }
    fn new_unnamed(value: MatchValue) -> Self {
        Self { value, name: None }
    }
    fn terminus() -> Self {
        Self {
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
    fn seg_type_match_string() -> Result<(), MatchError> {
        let result = SegType::match_string("hello")?;
        assert_eq!(result, MatchValue::String(String::from("hello")));
        Ok(())
    }
    #[test]
    fn seg_type_match_number_ok() -> Result<(), MatchError> {
        let result = SegType::match_number("123.45")?;
        assert_eq!(result, MatchValue::Number(123.45));
        Ok(())
    }
    #[test]
    fn seg_type_match_number_err() {
        let result = SegType::match_number("123.45.67");
        assert!(result.is_err());
    }
    #[test]
    fn seg_type_match_date_ok() -> Result<(), MatchError> {
        let result = SegType::match_date("2021-01-01")?;
        assert_eq!(result, MatchValue::Date(civil::date(2021, 1, 1)));
        Ok(())
    }
    #[test]
    fn seg_type_match_date_err() {
        let result = SegType::match_date("2021-01-01-01");
        assert!(result.is_err());
    }
    #[test]
    fn segment_static_match_ok() -> Result<(), MatchError> {
        let segment = Segment::Static("hello".to_string());
        let result = segment.match_segment("hello")?;
        assert_eq!(result.value, MatchValue::String("hello".to_string()));
        Ok(())
    }
    #[test]
    fn segment_static_match_err() -> Result<(), MatchError> {
        let segment = Segment::Static("hello".to_string());
        if let Err(MatchError::MatchError(exp, got)) = segment.match_segment("world") {
            assert_eq!(exp, "hello".to_string());
            assert_eq!(got, "world".to_string());
        } else {
            panic!("Expected error");
        }
        Ok(())
    }
    #[test]
    fn segment_terminus_match_ok() -> Result<(), MatchError> {
        let segment = Segment::Terminus;
        let result = segment.match_segment("")?;
        assert_eq!(result.value, MatchValue::Terminus);
        Ok(())
    }
    #[test]
    fn segment_terminus_match_err() -> Result<(), MatchError> {
        let segment = Segment::Terminus;
        if let Err(MatchError::MatchError(exp, got)) = segment.match_segment("world") {
            assert_eq!(exp, "<Terminus>".to_string());
            assert_eq!(got, "world".to_string());
        } else {
            panic!("Expected error");
        }
        Ok(())
    }
    #[test]
    fn segment_var_match_ok() -> Result<(), MatchError> {
        let segment = Segment::Var(Var::new("num".to_string(), SegType::Number));
        let result = segment.match_segment("123.45")?;
        assert_eq!(result.value, MatchValue::Number(123.45));
        Ok(())
    }
    #[test]
    fn segment_var_match_err() -> Result<(), MatchError> {
        let segment = Segment::Var(Var::new("num".to_string(), SegType::Number));
        if let Err(MatchError::NamedMatchError(name, exp, got)) = segment.match_segment("world") {
            assert_eq!(name, "num".to_string());
            assert_eq!(exp, "number".to_string());
            assert_eq!(got, "world".to_string());
        } else {
            panic!("Expected error");
        }
        Ok(())
    }
}
