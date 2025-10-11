use uuid::Uuid;

use crate::{auth::get_auth0_1, create_cook_and_run, team::post_test::create_team};

mod delete_test;
mod get_test;
mod post_test;

mod patch_test;

pub fn setup() -> (Uuid, Uuid) {
    let cook_and_run_id = create_cook_and_run();

    let (token, user_id) = get_auth0_1();
    let team_id = Uuid::new_v4();
    create_team(&cook_and_run_id, &team_id, &user_id, &token);

    (cook_and_run_id, team_id)
}
