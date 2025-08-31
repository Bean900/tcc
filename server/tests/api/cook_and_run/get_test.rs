use std::sync::{Mutex, OnceLock};

use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    auth::{get_auth0_1, get_auth0_2},
    cook_and_run::{
        assert_cook_and_run_json, assert_cook_and_run_meta_json,
        post_test::{create_cook_and_run, get_cook_and_run_create_json},
    },
    get_client,
};

static TEST_DATA: OnceLock<Mutex<TestData>> = OnceLock::new();

#[derive(Clone)]
pub struct TestData {
    creater_user: fn() -> (String, String),
    second_user: fn() -> (String, String),
    cook_and_run_id_list: Vec<Uuid>,
}

pub fn setup() -> TestData {
    let data = TEST_DATA.get_or_init(|| {
        let creater_user = get_auth0_1;
        let second_user = get_auth0_2;
        let mut cook_and_run_id_list = Vec::new();
        {
            let (token, user_id) = (creater_user)();
            {
                let (cook_and_run_id, payload) = get_cook_and_run_create_json(&user_id);
                create_cook_and_run(&cook_and_run_id, payload, &token);
                cook_and_run_id_list.push(cook_and_run_id);
            }
            {
                let (cook_and_run_id, payload) = get_cook_and_run_create_json(&user_id);
                create_cook_and_run(&cook_and_run_id, payload, &token);
                cook_and_run_id_list.push(cook_and_run_id);
            }
            {
                let (cook_and_run_id, payload) = get_cook_and_run_create_json(&user_id);
                create_cook_and_run(&cook_and_run_id, payload, &token);
                cook_and_run_id_list.push(cook_and_run_id);
            }
        }

        Mutex::new(TestData {
            creater_user,
            second_user,
            cook_and_run_id_list,
        })
    });

    data.lock().unwrap().clone()
}

#[test]
fn test_get_cook_and_run() {
    let test_data = setup();
    let (token, _) = (test_data.creater_user)();
    for cook_and_run_id in test_data.cook_and_run_id_list {
        get_cook_and_run(&cook_and_run_id, &token);
    }
}

#[test]
fn test_get_cook_and_run_meta() {
    let test_data = setup();
    let (token, user_id) = (test_data.creater_user)();
    get_cook_and_run_meta(&user_id, &token, test_data.cook_and_run_id_list.clone());
}

#[test]
fn test_get_cook_and_run_wrong_user() {
    let test_data = setup();
    let (token, _) = (test_data.second_user)();

    for cook_and_run_id in &test_data.cook_and_run_id_list {
        let res = execute_get(cook_and_run_id, &token);
        assert_eq!(
            res.status(),
            StatusCode::UNAUTHORIZED,
            "Response: {:#?}",
            res
        );
    }
}

#[test]
fn test_get_cook_and_run_meta_wrong_user() {
    let test_data = setup();
    let (token, _) = (test_data.second_user)();
    let (_, user_id) = (test_data.creater_user)();

    let res = execute_get_meta(&user_id, &token);
    assert_eq!(
        res.status(),
        StatusCode::UNAUTHORIZED,
        "Response: {:#?}",
        res
    );
}

fn execute_get(cook_and_run_id: &Uuid, token: &str) -> reqwest::blocking::Response {
    let (client, base_url) = get_client();
    client
        .get(&format!("{}/cook_and_run/{}", base_url, cook_and_run_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .expect("Failed to send request")
}

pub fn get_cook_and_run(cook_and_run_id: &Uuid, token: &str) {
    let res = execute_get(cook_and_run_id, token);
    assert!(res.status().is_success(), "Response: {:#?}", res);
    assert_cook_and_run_json(res.json().expect("Failed to parse JSON"), cook_and_run_id);
}

fn execute_get_meta(user_id: &str, token: &str) -> reqwest::blocking::Response {
    let (client, base_url) = get_client();
    client
        .get(&format!("{}/cook_and_run?userId={}", base_url, user_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .expect("Failed to send request")
}

pub fn get_cook_and_run_meta(user_id: &str, token: &str, expected_cook_and_run_id: Vec<Uuid>) {
    let res = execute_get_meta(user_id, token);
    assert!(res.status().is_success(), "Response: {:#?}", res);
    assert_cook_and_run_meta_json(
        res.json().expect("Failed to parse JSON"),
        expected_cook_and_run_id,
    );
}
