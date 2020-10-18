use json_patch::Operation;
use json_patch::Patch;
use serde_json::from_str;

#[test]
fn test_structure() {
  let json: &str = r#"
    [
      {"op": "test", "path": "/a/b/c", "value": "foo"},
      {"op": "remove", "path": "/a/b/c"},
      {"op": "add", "path": "/a/b/c", "value": ["foo", "bar"]},
      {"op": "replace", "path": "/a/b/c", "value": 42},
      {"op": "move", "from": "/a/b/c", "path": "/a/b/d"},
      {"op": "copy", "from": "/a/b/d", "path": "/a/b/e"}
    ]
  "#;

  assert!(from_str::<Patch>(json).is_ok());

  let json: &str = r#"{"op": "add", "path": "/a/b/c", "value": [ "foo", "bar" ]}"#;
  assert!(matches!(from_str(json).unwrap(), Operation::Add(_)));

  let json: &str = r#"{"op": "remove", "path": "/a/b/c"}"#;
  assert!(matches!(from_str(json).unwrap(), Operation::Remove(_)));

  let json: &str = r#"{"op": "replace", "path": "/a/b/c", "value": 42}"#;
  assert!(matches!(from_str(json).unwrap(), Operation::Replace(_)));

  let json: &str = r#"{"op": "move", "from": "/a/b/c", "path": "/a/b/d"}"#;
  assert!(matches!(from_str(json).unwrap(), Operation::Move(_)));

  let json: &str = r#"{"op": "copy", "from": "/a/b/c", "path": "/a/b/e"}"#;
  assert!(matches!(from_str(json).unwrap(), Operation::Copy(_)));

  let json: &str = r#"{"op": "test", "path": "/a/b/c", "value": "foo"}"#;
  assert!(matches!(from_str(json).unwrap(), Operation::Test(_)));
}
