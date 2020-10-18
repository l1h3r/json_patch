/// A JSON Patch "remove" operation.
///
/// Removes the value at the target location.
///
/// [More Info](https://tools.ietf.org/html/rfc6902#section-4.2)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OpRemove {
  /// A string containing a JSON-Pointer value that references a location within
  /// the target document (the "target location") where the operation is
  /// performed.
  pub path: String,
}
