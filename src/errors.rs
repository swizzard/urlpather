use thiserror::Error;
#[derive(Error, Debug)]
pub enum ParserConfigError {
    #[error("Invalid segment type")]
    InvalidSegmentType,
}
