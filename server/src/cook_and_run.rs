use chrono::NaiveDateTime;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
    address::{self, Address},
    course::{self, Course},
    db::{self, Database},
    plan::{self, Plan},
    sharing::{self, ShareTeamConfig},
    team::{self, Team},
};

// Cook and Run models
#[derive(Debug, Clone)]
pub struct CookAndRunMeta {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
}

impl CookAndRunMeta {
    fn from(cook_and_run: db::models::CookAndRun) -> Self {
        CookAndRunMeta {
            id: cook_and_run.id,
            name: cook_and_run.name,
            created: cook_and_run.created,
            edited: cook_and_run.edited,
            occur: cook_and_run.occur,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CookAndRun {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
    pub team_list: Vec<Team>,
    pub course_list: Vec<Course>,
    pub course_with_multiple_hosts: Option<Uuid>,
    pub start_point: Option<Address>,
    pub end_point: Option<Address>,
    pub share_team_config: Option<ShareTeamConfig>,
    pub plan: Option<Plan>,
}

impl CookAndRun {
    fn from(
        cook_and_run: db::models::CookAndRun,
        team_list: Vec<Team>,
        course_list: Vec<Course>,
        start_point: Option<Address>,
        end_point: Option<Address>,
        share_team_config: Option<ShareTeamConfig>,
        plan: Option<Plan>,
    ) -> Self {
        CookAndRun {
            id: cook_and_run.id,
            name: cook_and_run.name,
            created: cook_and_run.created,
            edited: cook_and_run.edited,
            occur: cook_and_run.occur,
            team_list,
            course_list,
            course_with_multiple_hosts: cook_and_run.course_with_multiple_hosts,
            start_point,
            end_point,
            share_team_config,
            plan,
        }
    }
}

pub fn get_list_of_cook_and_run_meta(
    db: &mut Database,
    user_id: &str,
) -> Result<Vec<CookAndRunMeta>, String> {
    db.select_all_cook_and_run(user_id)
        .map(|list| list.into_iter().map(CookAndRunMeta::from).collect())
        .map_err(|e| {
            event!(
                Level::WARN,
                "DB Error while loading all cook and run meta for user {}: {}",
                user_id,
                e
            );
            e
        })
}

pub fn get_cook_and_run(db: &mut Database, cook_and_run_id: &Uuid) -> Result<CookAndRun, String> {
    let cook_and_run = db.select_cook_and_run(cook_and_run_id)?;
    let team = team::get_list(db, cook_and_run_id)?;
    let course = course::get_list(db, cook_and_run_id)?;

    let start_point = cook_and_run
        .start_point
        .map(|a| address::get_by_id(db, &a))
        .transpose()?;

    let end_point = cook_and_run
        .end_point
        .map(|a| address::get_by_id(db, &a))
        .transpose()?;

    let share_team_config = cook_and_run
        .share_team_config
        .map(|s| sharing::get_by_id(db, &s))
        .transpose()?;

    let plan = cook_and_run
        .plan
        .map(|p| plan::get_by_id(db, &p))
        .transpose()?;

    Ok(CookAndRun::from(
        cook_and_run,
        team,
        course,
        start_point,
        end_point,
        share_team_config,
        plan,
    ))
}
