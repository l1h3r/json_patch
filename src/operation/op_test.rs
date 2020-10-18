use serde_json::Value;

/// A JSON Patch "test" operation.
///
/// Tests that a value at the target location is equal to a specified value.
///
/// [More Info](https://tools.ietf.org/html/rfc6902#section-4.6)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OpTest {
  /// A string containing a JSON-Pointer value that references a location within
  /// the target document (the "target location") where the operation is
  /// performed.
  pub path: String,
  /// The value to be compared to the target location's value.
  pub value: Value,
}
