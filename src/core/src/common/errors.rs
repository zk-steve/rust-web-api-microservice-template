#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    NotFound,
    InternalError,
}
