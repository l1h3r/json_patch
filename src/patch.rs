use serde_json::Value;

use crate::error::Result;
use crate::operation::Operation;
use crate::traits::CanPatch;

/// A sequence of JSON Patch operations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Patch(Vec<Operation>);

impl Patch {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn apply_ref(self, output: &Value) -> Result<Value> {
    let mut value: Value = output.clone();

    self.apply_mut(&mut value)?;

    Ok(value)
  }

  pub fn apply_mut(self, output: &mut Value) -> Result<()> {
    if self.is_empty() {
      return Ok(());
    }

    if !matches!(output, Value::Object(_) | Value::Array(_)) {
      *output = Value::Object(Default::default());
    }

    for operation in self.0 {
      operation.apply(output)?;
    }

    Ok(())
  }

  pub fn apply_fun(self, mut f: impl CanPatch) -> Result<()> {
    for operation in self.0 {
      match operation {
        Operation::Add(op) => f.patch_add(op)?,
        Operation::Remove(op) => f.patch_remove(op)?,
        Operation::Replace(op) => f.patch_replace(op)?,
        Operation::Move(op) => f.patch_move(op)?,
        Operation::Copy(op) => f.patch_copy(op)?,
        Operation::Test(op) => f.patch_test(op)?,
      }
    }

    Ok(())
  }
}
