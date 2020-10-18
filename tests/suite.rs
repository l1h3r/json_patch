// ==
// https://github.com/json-patch/json-patch-tests
// ==
use serde::Deserialize;
use serde_json::from_slice;
use serde_json::from_value;
use serde_json::Value;

use json_patch::Error;
use json_patch::Patch;

const T1: &[u8] = include_bytes!("fixtures/spec_tests.json");
const T2: &[u8] = include_bytes!("fixtures/tests.json");

#[derive(Clone, Debug, Deserialize)]
struct Test {
  #[serde(default)]
  comment: String,
  #[serde(default)]
  disabled: bool,
  doc: Value,
  patch: Value,
  error: Option<String>,
  expected: Option<Value>,
}

const INVALID_POINTER: &[&str] = &[
  // tests.json
  "Out of bounds (upper)",
  "Out of bounds (lower)",
  "index is greater than number of items in array",
  "Object operation on array target",
  "replace op should fail with missing parent key",
  "remove op shouldn't remove from array with bad number",
  "replace op shouldn't replace in array with bad number",
  "copy op shouldn't work with bad number",
  "move op shouldn't work with bad number",
  "add op shouldn't add to array with bad number",
  "JSON Pointer should start with a slash",
  "missing 'from' location",
  "removing a nonexistent field should fail",
  "removing a nonexistent index should fail",
  // spec_tests.json
  "path /a does not exist -- missing objects are not created recursively",
  "add to a non-existent target",
];

const INVALID_TEST: &[&str] = &[
  // tests.json
  "test op should fail",
  "test op shouldn't get array element 1",
  "test op should reject the array value, it has leading zeros",
  // spec_tests.json
  "string not equivalent",
  "number is not equal to string",
];

fn check_err(expected: String, current: String) {
  if INVALID_POINTER.contains(&&*expected) {
    assert_eq!(current, Error::InvalidPointer.to_string());
  } else if INVALID_TEST.contains(&&*expected) {
    assert_eq!(current, Error::InvalidTest.to_string());
  } else {
    match expected.as_str() {
      "missing 'from' parameter" => {
        assert_eq!(current, "missing field `from`");
      }
      "missing 'path' parameter" => {
        assert_eq!(current, "missing field `path`");
      }
      "missing 'value' parameter" => {
        assert_eq!(current, "missing field `value`");
      }
      "null is not valid value for 'path'" => {
        assert_eq!(current, "invalid type: null, expected a string");
      }
      "Unrecognized op 'spam'" => {
        assert_eq!(current, "unknown variant `spam`, expected one of `add`, `remove`, `replace`, `move`, `copy`, `test`");
      }
      _ => {
        panic!("Unknown Error: {:?} != {:?}", current, expected);
      }
    }
  }
}

fn check_test(index: usize, mut test: Test) {
  if test.disabled {
    println!("[+] Test #{:02} (D): {}", index, test.comment);
  } else {
    println!("[+] Test #{:02} (E): {}", index, test.comment);

    match (test.expected, test.error) {
      (Some(inner), None) => {
        from_value::<Patch>(test.patch)
          .unwrap()
          .apply_mut(&mut test.doc)
          .unwrap();

        assert_eq!(inner, test.doc);
      }
      (None, Some(inner)) => match from_value::<Patch>(test.patch) {
        Ok(patch) => check_err(
          inner,
          patch.apply_mut(&mut test.doc).unwrap_err().to_string(),
        ),
        Err(error) => check_err(inner, error.to_string()),
      },
      (Some(_), Some(_)) | (None, None) => unreachable!(),
    }
  }
}

#[test]
fn test_tests() {
  for (index, test) in from_slice::<Vec<Test>>(T1).unwrap().into_iter().enumerate() {
    check_test(index, test);
  }

  for (index, test) in from_slice::<Vec<Test>>(T2).unwrap().into_iter().enumerate() {
    check_test(index, test);
  }
}
