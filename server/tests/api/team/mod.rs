use uuid::Uuid;

use crate::{auth::get_auth0_1, create_cook_and_run, team::post_test::create_team};

mod delete_test;
mod get_test;
mod patch_test;
mod post_test;

mod note;

pub fn setup() -> (Uuid, Uuid) {
    let cook_and_run_id = create_cook_and_run();

    let (token, user_id) = get_auth0_1();
    let team_id = Uuid::new_v4();
    create_team(&cook_and_run_id, &team_id, &user_id, &token);

    (cook_and_run_id, team_id)
}

pub fn get_team(cook_and_run_id: &Uuid, team_id: &Uuid) -> serde_json::Value {
    let (token, _) = get_auth0_1();
    let res = get_test::execute_get(&cook_and_run_id, team_id, &token);
    assert!(res.status().is_success(), "Response: {:#?}", res);
    res.json().expect("Failed to parse JSON")
}
