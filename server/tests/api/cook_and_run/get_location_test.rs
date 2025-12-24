use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    auth::{get_auth0_1, get_auth0_2},
    cook_and_run::patch_location_test::{
        patch_end_point_cook_and_run, patch_start_point_cook_and_run,
    },
    create_cook_and_run, get_client,
};

#[test]
fn test_get_start_point_cook_and_run() {
    let (token, _) = get_auth0_1();
    let cook_and_run_id = create_cook_and_run();
    let start_addr = patch_start_point_cook_and_run(&cook_and_run_id, &token);
    let res = execute_get_start_point(&cook_and_run_id, &token);
    assert!(res.status().is_success(), "Response: {:#?}", res);
    assert_address_json(res.json().expect("Failed to parse JSON"), &start_addr);
}

#[test]
fn test_get_start_point_cook_and_run_not_found() {
    let (token, _) = get_auth0_1();
    let cook_and_run_id = create_cook_and_run();
    let res = execute_get_start_point(&cook_and_run_id, &token);
    assert_eq!(res.status(), StatusCode::NO_CONTENT, "Response: {:#?}", res);
}

#[test]
fn test_get_start_point_cook_and_run_not_existing() {
    let (token, _) = get_auth0_1();
    let res = execute_get_start_point(&Uuid::new_v4(), &token);
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Response: {:#?}", res);
}

#[test]
fn test_get_start_point_cook_and_run_not_authorised() {
    let (token_1, _) = get_auth0_1();
    let cook_and_run_id = create_cook_and_run();
    let (token_2, _) = get_auth0_2();
    let res = execute_get_start_point(&cook_and_run_id, &token_2);
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Response: {:#?}", res);

    let _ = patch_start_point_cook_and_run(&cook_and_run_id, &token_1);

    let res = execute_get_start_point(&cook_and_run_id, &token_2);
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Response: {:#?}", res);
}

#[test]
fn test_get_end_point_cook_and_run() {
    let (token, _) = get_auth0_1();
    let cook_and_run_id = create_cook_and_run();
    let end_addr = patch_end_point_cook_and_run(&cook_and_run_id, &token);
    let res = execute_get_end_point(&cook_and_run_id, &token);
    assert!(res.status().is_success(), "Response: {:#?}", res);
    assert_address_json(res.json().expect("Failed to parse JSON"), &end_addr);
}

#[test]
fn test_get_end_point_cook_and_run_not_found() {
    let (token, _) = get_auth0_1();
    let cook_and_run_id = create_cook_and_run();
    let res = execute_get_end_point(&cook_and_run_id, &token);
    assert_eq!(res.status(), StatusCode::NO_CONTENT, "Response: {:#?}", res);
}

#[test]
fn test_get_end_point_cook_and_run_not_existing() {
    let (token, _) = get_auth0_1();
    let res = execute_get_end_point(&Uuid::new_v4(), &token);
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Response: {:#?}", res);
}

#[test]
fn test_get_end_point_cook_and_run_not_authorised() {
    let (token_1, _) = get_auth0_1();
    let cook_and_run_id = create_cook_and_run();
    let (token_2, _) = get_auth0_2();
    let res = execute_get_end_point(&cook_and_run_id, &token_2);
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Response: {:#?}", res);

    let _ = patch_end_point_cook_and_run(&cook_and_run_id, &token_1);

    let res = execute_get_end_point(&cook_and_run_id, &token_2);
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Response: {:#?}", res);
}

pub fn execute_get_start_point(cook_and_run_id: &Uuid, token: &str) -> reqwest::blocking::Response {
    let (client, base_url) = get_client();
    client
        .get(&format!(
            "{}/cook_and_run/{}/start_point",
            base_url, cook_and_run_id
        ))
        .header("authorization", format!("Bearer {}", token))
        .send()
        .expect("Failed to send request")
}

pub fn execute_get_end_point(cook_and_run_id: &Uuid, token: &str) -> reqwest::blocking::Response {
    let (client, base_url) = get_client();
    client
        .get(&format!(
            "{}/cook_and_run/{}/end_point",
            base_url, cook_and_run_id
        ))
        .header("authorization", format!("Bearer {}", token))
        .send()
        .expect("Failed to send request")
}

pub fn assert_address_json(json: serde_json::Value, addr: &str) {
    let address = json
        .get("address")
        .and_then(|v| v.as_str())
        .expect("Missing id");
    let latitude = json
        .get("latitude")
        .and_then(|v| v.as_f64())
        .expect("Missing latitude");
    let longitude = json
        .get("longitude")
        .and_then(|v| v.as_f64())
        .expect("Missing longitude");

    assert_eq!(address, addr, "Address does not match. Response: {}", json);

    assert_eq!(
        latitude, 48.137154,
        "Latitude does not match. Response: {}",
        json
    );

    assert_eq!(
        longitude, 11.57549,
        "Longitude does not match. Response: {}",
        json
    );
}
