use serde_json::Value;

pub fn merge_ref(value: &Value, patch: &Value) -> Value {
  let mut value: Value = value.clone();

  merge_mut(&mut value, patch);

  value
}

pub fn merge_mut(value: &mut Value, patch: &Value) {
  match (value, patch) {
    (Value::Object(value), Value::Object(patch)) => {
      for (name, other) in patch {
        if other.is_null() {
          value.remove(name.as_str());
        } else {
          merge_mut(value.entry(name.as_str()).or_insert(Value::Null), other);
        }
      }
    }
    (value @ Value::Object(_), _) => {
      *value = patch.clone();
    }
    (value, _) => {
      *value = Value::Object(Default::default());
      merge_mut(value, patch);
    }
  }
}
