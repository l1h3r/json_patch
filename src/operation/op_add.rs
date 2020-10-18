use serde_json::Value;

/// A JSON Patch "add" operation
///
/// Adds a value at the target location.
///
/// [More Info](https://tools.ietf.org/html/rfc6902#section-4.1)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OpAdd {
  /// A string containing a JSON-Pointer value that references a location within
  /// the target document (the "target location") where the operation is
  /// performed.
  pub path: String,
  /// The value to be added.
  pub value: Value,
}
