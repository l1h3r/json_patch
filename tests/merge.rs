use json_patch::merge_ref;
use serde_json::json;

#[test]
#[rustfmt::skip]
fn test_rfc7396() {
  assert_eq!(merge_ref(&json!({"a":"b"}), &json!({"a":"c"})), json!({"a":"c"}));
  assert_eq!(merge_ref(&json!({"a":"b"}), &json!({"b":"c"})), json!({"a":"b","b":"c"}));
  assert_eq!(merge_ref(&json!({"a":"b"}), &json!({ "a": null })), json!({}));
  assert_eq!(merge_ref(&json!({"a":"b","b":"c"}), &json!({ "a": null })), json!({"b":"c"}));
  assert_eq!(merge_ref(&json!({"a":["b"]}), &json!({"a":"c"})), json!({"a":"c"}));
  assert_eq!(merge_ref(&json!({"a":"c"}), &json!({"a":["b"]})), json!({"a":["b"]}));
  assert_eq!(merge_ref(&json!({"a": {"b": "c"}}), &json!({"a": {"b": "d","c": null}})), json!({"a": {"b": "d"}}));
  assert_eq!(merge_ref(&json!({"a": [{"b":"c"}]}), &json!({"a": [1]})), json!({"a": [1]}));
  assert_eq!(merge_ref(&json!(["a", "b"]), &json!(["c", "d"])), json!(["c", "d"]));
  assert_eq!(merge_ref(&json!({"a":"b"}), &json!(["c"])), json!(["c"]));
  assert_eq!(merge_ref(&json!({"a":"foo"}), &json!(null)), json!(null));
  assert_eq!(merge_ref(&json!({"a":"foo"}), &json!("bar")), json!("bar"));
  assert_eq!(merge_ref(&json!({ "e": null }), &json!({"a":1})), json!({"e":null,"a":1}));
  assert_eq!(merge_ref(&json!([1, 2]), &json!({"a":"b","c":null})), json!({"a":"b"}));
  assert_eq!(merge_ref(&json!({}), &json!({"a":{"bb":{"ccc":null}}})), json!({"a":{"bb":{}}}));
}
