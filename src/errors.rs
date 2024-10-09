use thiserror::Error;
#[derive(Error, Debug)]
pub enum ParserConfigError {
    #[error("Invalid segment type")]
    InvalidSegmentType,
}

#[derive(Error, Debug)]
pub enum MatchError {
    #[error("Match error: expected {0}, got {1}")]
    MatchError(String, String),
    #[error("Match error at {0}: expected {1}, got {2}")]
    NamedMatchError(String, String, String),
}

impl MatchError {
    pub fn with_name(self, name: String) -> Self {
        match self {
            MatchError::MatchError(s1, s2) => MatchError::NamedMatchError(name, s1, s2),
            MatchError::NamedMatchError(_, _, _) => self,
        }
    }
}
