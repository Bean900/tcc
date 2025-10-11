use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::{
    auth::{get_auth0_1, get_auth0_2},
    create_cook_and_run, get_client,
};

#[test]
fn test_create_team() {
    let cook_and_run_id = create_cook_and_run();
    let team_id = Uuid::new_v4();

    let (token, user_id) = get_auth0_1();
    create_team(&cook_and_run_id, &team_id, &user_id, &token);
}

#[test]
fn test_create_created_team() {
    let cook_and_run_id = create_cook_and_run();
    let team_id = Uuid::new_v4();

    let (token, user_id) = get_auth0_1();
    create_team(&cook_and_run_id, &team_id, &user_id, &token);
    create_team(&cook_and_run_id, &team_id, &user_id, &token);
}

#[test]
fn test_create_team_wrong_user() {
    let cook_and_run_id = create_cook_and_run();
    let team_id = Uuid::new_v4();
    let (token, user_id) = get_auth0_2();

    let payload = get_team_create_json(&user_id);
    let res = execute_create(&cook_and_run_id, &team_id, payload, &token);

    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Response: {:#?}", res);
}

fn execute_create(
    cook_and_run_id: &Uuid,
    team_id: &Uuid,
    payload: serde_json::Value,
    token: &str,
) -> reqwest::blocking::Response {
    let (client, base_url) = get_client();
    client
        .post(&format!(
            "{}/cook_and_run/{}/team/{}",
            base_url, cook_and_run_id, team_id
        ))
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()
        .expect("Failed to send request")
}

pub fn create_team(cook_and_run_id: &Uuid, team_id: &Uuid, user_id: &str, token: &str) {
    let payload = get_team_create_json(user_id);
    let res = execute_create(cook_and_run_id, team_id, payload, token);
    assert!(res.status().is_success(), "Response: {:#?}", res);
}

pub fn get_team_create_json(user_id: &str) -> serde_json::Value {
    let json = json!({
        "userId": user_id,
        "name": "TestTeam",
        "address": {
            "address": "Hasengasse 5-7, 60311 Frankfurt am Main, Deutschland",
            "latitude": 50.11278393458553,
            "longitude":8.682874268586934,
        },
        "members": 2,
        "mail":"cook@run.de",
        "phone":"+49 12345",
        "diets": "No special diets",
        "needs_check":true,
    });
    json
}
