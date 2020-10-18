use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
  /// The result of accessing an invalid JSON-Pointer.
  InvalidPointer,
  /// The result of a failed `test` operation.
  InvalidTest,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Self::InvalidPointer => f.write_str("Invalid JSON Pointer"),
      Self::InvalidTest => f.write_str("Test Operation Failed"),
    }
  }
}
