extern crate jsonpath_lib as jsonpath;
#[macro_use]
extern crate serde_json;


fn main() {
    let json_obj = json!({
        "school": {
            "friends": [{"id": 0}, {"id": 1}]
        },
        "friends": [{"id": 0}, {"id": 1}]
    });
    let json = jsonpath::select(&json_obj, "$..friends[0]").unwrap();
    let ret = json!({"id": 0});
    assert_eq!(json[0], &ret);
}
