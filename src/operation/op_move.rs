/// A JSON Patch "move" operation.
///
/// Removes the value at a specified location and adds it to the target location.
///
/// [More Info](https://tools.ietf.org/html/rfc6902#section-4.4)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OpMove {
  /// A string containing a JSON-Pointer value that references a location within
  /// the target document (the "target location") where the operation is
  /// performed.
  pub path: String,
  /// A string containing a JSON-Pointer value that references the location in
  /// the target document to move the value from.
  pub from: String,
}
