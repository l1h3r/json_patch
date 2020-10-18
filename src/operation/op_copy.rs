/// A JSON Patch "copy" operation.
///
/// Copies the value at a specified location to the target location.
///
/// [More Info](https://tools.ietf.org/html/rfc6902#section-4.5)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OpCopy {
  /// A string containing a JSON-Pointer value that references a location within
  /// the target document (the "target location") where the operation is
  /// performed.
  pub path: String,
  /// A string containing a JSON-Pointer value that references the location in
  /// the target document to copy the value from.
  pub from: String,
}
