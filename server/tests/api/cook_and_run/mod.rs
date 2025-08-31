use uuid::Uuid;

mod get_test;
mod post_test;

fn assert_cook_and_run_json(json: serde_json::Value, cook_and_run_id: &Uuid) {
    let id = json.get("id").and_then(|v| v.as_str()).expect("Missing id");
    let name = json
        .get("name")
        .and_then(|v| v.as_str())
        .expect("Missing name");

    assert_eq!(
        id,
        cook_and_run_id.to_string(),
        "Cook and Run ID does not match"
    );
    assert_eq!(name, "Test Cook & Run", "Cook and Run name does not match");
}

fn assert_cook_and_run_meta_json(json: serde_json::Value, cook_and_run_id_list: Vec<Uuid>) {
    let data = json
        .get("data")
        .and_then(|v| v.as_array())
        .expect("Missing data array");
    let json_ids: Vec<Uuid> = data
        .iter()
        .map(|item| {
            item.get("id")
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
                .expect("Invalid or missing id in data item")
        })
        .collect();

    for expected_id in cook_and_run_id_list {
        assert!(
            json_ids.contains(&expected_id),
            "Expected Cook and Run ID {} not found in JSON data",
            expected_id
        );
    }
}
