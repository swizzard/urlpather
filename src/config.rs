use crate::errors::ParserConfigError;
use crate::segments::SegType;

/// TODO(SHR): Replace this with real parsing after we settle on format
/// in the meantime, notes:

impl TryFrom<&str> for SegType {
    type Error = ParserConfigError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "" => Ok(SegType::String),
            "number" => Ok(SegType::Number),
            "string" => Ok(SegType::String),
            "date" => Ok(SegType::Date),
            _ => Err(ParserConfigError::InvalidSegmentType),
        }
    }
}

// static segments can't contain / or other url-invalid chars

// impl TryFrom<&str> for Var {
//     type Error = ParserConfigError;
//
//     fn try_from(value: &str) -> Result<Self, Self::Error> {}
// }
