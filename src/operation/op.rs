use core::mem;
use serde_json::Value;

use crate::error::Error;
use crate::error::Result;
use crate::operation::OpAdd;
use crate::operation::OpCopy;
use crate::operation::OpMove;
use crate::operation::OpRemove;
use crate::operation::OpReplace;
use crate::operation::OpTest;

/// A JSON Patch operation.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "op")]
pub enum Operation {
  Add(OpAdd),
  Remove(OpRemove),
  Replace(OpReplace),
  Move(OpMove),
  Copy(OpCopy),
  Test(OpTest),
}

impl Operation {
  pub fn apply(self, value: &mut Value) -> Result<()> {
    match self {
      Operation::Add(op) => {
        let _: Option<Value> = add(value, op)?;
      }
      Operation::Remove(op) => {
        let _: Value = remove(value, op)?;
      }
      Operation::Replace(op) => {
        let _: Value = replace(value, op)?;
      }
      Operation::Move(op) => {
        let _: Option<Value> = move_(value, op)?;
      }
      Operation::Copy(op) => {
        let _: Option<Value> = copy(value, op)?;
      }
      Operation::Test(op) => {
        let _: () = test(value, op)?;
      }
    }

    Ok(())
  }
}

fn pointer(value: &Value, pointer: impl AsRef<str>) -> Result<&Value> {
  value.pointer(pointer.as_ref()).ok_or(Error::InvalidPointer)
}

fn pointer_mut(value: &mut Value, pointer: impl AsRef<str>) -> Result<&mut Value> {
  value
    .pointer_mut(pointer.as_ref())
    .ok_or(Error::InvalidPointer)
}

fn split_at(pointer: &str) -> Result<(&str, &str)> {
  pointer
    .rfind('/')
    .ok_or(Error::InvalidPointer)
    .map(|index| (&pointer[..index], &pointer[index + 1..]))
}

fn parse_idx(value: &str, limit: usize) -> Result<usize> {
  // * If the currently referenced value is a JSON array, the reference
  //   token MUST contain either:

  //   * characters comprised of digits (see ABNF below; note that
  //     leading zeros are not allowed) that represent an unsigned
  //     base-10 integer value, making the new referenced value the
  //     array element with the zero-based index identified by the
  //     token, or

  //   * exactly the single character "-", making the new referenced
  //     value the (nonexistent) member after the last array element.
  if value.starts_with('0') && value.len() != 1 {
    return Err(Error::InvalidPointer);
  }

  match value.parse() {
    Ok(index) if index < limit => Ok(index),
    Ok(_) | Err(_) => Err(Error::InvalidPointer),
  }
}

fn add(output: &mut Value, op: OpAdd) -> Result<Option<Value>> {
  // The "add" operation performs one of the following functions,
  // depending upon what the target location references:
  //
  // * If the target location specifies an array index, a new value is
  //   inserted into the array at the specified index.
  //
  // * If the target location specifies an object member that does not
  //   already exist, a new member is added to the object.
  //
  // * If the target location specifies an object member that does exist,
  //   that member's value is replaced.
  //
  // The operation object MUST contain a "value" member whose content
  // specifies the value to be added.
  //
  // For example:
  //
  // { "op": "add", "path": "/a/b/c", "value": [ "foo", "bar" ] }
  if op.path.is_empty() {
    Ok(Some(mem::replace(output, op.value)))
  } else {
    let (ptr, key) = split_at(&op.path)?;

    match pointer_mut(output, ptr)? {
      Value::Object(inner) => Ok(inner.insert(key.into(), op.value)),
      Value::Array(inner) if key == "-" => {
        inner.push(op.value);
        Ok(None)
      }
      Value::Array(inner) => {
        inner.insert(parse_idx(key, inner.len() + 1)?, op.value);
        Ok(None)
      }
      _ => Err(Error::InvalidPointer),
    }
  }
}

fn remove(output: &mut Value, op: OpRemove) -> Result<Value> {
  // The "remove" operation removes the value at the target location.
  //
  // The target location MUST exist for the operation to be successful.
  //
  // For example:
  //
  // { "op": "remove", "path": "/a/b/c" }
  //
  // If removing an element from an array, any elements above the
  // specified index are shifted one position to the left.
  let (ptr, key) = split_at(&op.path)?;

  match pointer_mut(output, ptr)? {
    Value::Object(inner) => inner.remove(key).ok_or(Error::InvalidPointer),
    Value::Array(inner) => Ok(inner.remove(parse_idx(key, inner.len())?)),
    _ => Err(Error::InvalidPointer),
  }
}

fn replace(output: &mut Value, op: OpReplace) -> Result<Value> {
  // The "replace" operation replaces the value at the target location
  // with a new value. The operation object MUST contain a "value" member
  // whose content specifies the replacement value.
  //
  // The target location MUST exist for the operation to be successful.
  //
  // For example:
  //
  // { "op": "replace", "path": "/a/b/c", "value": 42 }
  //
  // This operation is functionally identical to a "remove" operation for
  // a value, followed immediately by an "add" operation at the same
  // location with the replacement value.
  pointer_mut(output, &op.path).map(|other| mem::replace(other, op.value))
}

fn move_(output: &mut Value, op: OpMove) -> Result<Option<Value>> {
  // The "move" operation removes the value at a specified location and
  // adds it to the target location.
  //
  // The operation object MUST contain a "from" member, which is a string
  // containing a JSON Pointer value that references the location in the
  // target document to move the value from.
  //
  // The "from" location MUST exist for the operation to be successful.
  //
  // For example:
  //
  // { "op": "move", "from": "/a/b/c", "path": "/a/b/d" }
  //
  // This operation is functionally identical to a "remove" operation on
  // the "from" location, followed immediately by an "add" operation at
  // the target location with the value that was just removed.
  //
  // The "from" location MUST NOT be a proper prefix of the "path"
  // location; i.e., a location cannot be moved into one of its children.
  if op.path.starts_with(&op.from) && op.path[op.from.len()..].starts_with('/') {
    return Err(Error::InvalidPointer);
  }

  let value: Value = remove(output, OpRemove { path: op.from })?;

  add(
    output,
    OpAdd {
      path: op.path,
      value,
    },
  )
}

fn copy(output: &mut Value, op: OpCopy) -> Result<Option<Value>> {
  // The "copy" operation copies the value at a specified location to the
  // target location.
  //
  // The operation object MUST contain a "from" member, which is a string
  // containing a JSON Pointer value that references the location in the
  // target document to copy the value from.
  //
  // The "from" location MUST exist for the operation to be successful.
  //
  // For example:
  //
  // { "op": "copy", "from": "/a/b/c", "path": "/a/b/e" }
  //
  // This operation is functionally identical to an "add" operation at the
  // target location using the value specified in the "from" member.
  let value: Value = pointer(output, &op.from)?.clone();

  add(
    output,
    OpAdd {
      path: op.path,
      value,
    },
  )
}

fn test(output: &Value, op: OpTest) -> Result<()> {
  // The "test" operation tests that a value at the target location is
  // equal to a specified value.
  //
  // The operation object MUST contain a "value" member that conveys the
  // value to be compared to the target location's value.
  //
  // The target location MUST be equal to the "value" value for the
  // operation to be considered successful.
  //
  // Here, "equal" means that the value at the target location and the
  // value conveyed by "value" are of the same JSON type, and that they
  // are considered equal by the following rules for that type:
  //
  // o  strings: are considered equal if they contain the same number of
  //    Unicode characters and their code points are byte-by-byte equal.
  //
  // o  numbers: are considered equal if their values are numerically
  //    equal.
  //
  // o  arrays: are considered equal if they contain the same number of
  //    values, and if each value can be considered equal to the value at
  //    the corresponding position in the other array, using this list of
  //    type-specific rules.
  //
  // o  objects: are considered equal if they contain the same number of
  //    members, and if each member can be considered equal to a member in
  //    the other object, by comparing their keys (as strings) and their
  //    values (using this list of type-specific rules).
  //
  // o  literals (false, true, and null): are considered equal if they are
  //    the same.
  //
  // Note that the comparison that is done is a logical comparison; e.g.,
  // whitespace between the member values of an array is not significant.
  //
  // Also, note that ordering of the serialization of object members is
  // not significant.
  //
  // For example:
  //
  // { "op": "test", "path": "/a/b/c", "value": "foo" }
  if matches!(pointer(output, op.path), Ok(pointer) if *pointer == op.value) {
    Ok(())
  } else {
    Err(Error::InvalidTest)
  }
}
