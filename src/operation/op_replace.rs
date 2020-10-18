use serde_json::Value;

/// A JSON Patch "replace" operation.
///
/// Replaces the value at the target location with a new value.
///
/// [More Info](https://tools.ietf.org/html/rfc6902#section-4.3)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OpReplace {
  /// A string containing a JSON-Pointer value that references a location within
  /// the target document (the "target location") where the operation is
  /// performed.
  pub path: String,
  /// The replacement value.
  pub value: Value,
}
