use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::rest::auth::{AuthUser, AuthenticatedUser};

// Common types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl PaginationInfo {
    pub fn new() -> Self {
        PaginationInfo {
            page: 1 as u32,
            limit: 1 as u32,
            total: 1 as u64,
            total_pages: 1 as u32,
            has_next: false,
            has_prev: false,
        }
    }
}

// Address model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Address {
    pub fn from(address: crate::address::Address) -> Self {
        Address {
            address: address.address,
            latitude: address.latitude,
            longitude: address.longitude,
        }
    }

    pub fn to(&self) -> crate::address::Address {
        crate::address::Address {
            id: Uuid::new_v4(),
            address: self.address.clone(),
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

// Cook and Run models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookAndRunMeta {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
}

impl CookAndRunMeta {
    pub fn from(cook_and_run: &crate::cook_and_run::CookAndRunMeta) -> Self {
        CookAndRunMeta {
            id: cook_and_run.id,
            user_id: cook_and_run.user_id.clone(),
            name: cook_and_run.name.clone(),
            created: cook_and_run.created,
            edited: cook_and_run.edited,
            occur: cook_and_run.occur,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookAndRunCreateData {
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl CookAndRunCreateData {
    pub fn to_cook_and_run_create<'a>(
        &'a self,
        cook_and_run_id: &'a Uuid,
        time: &'a NaiveDateTime,
    ) -> crate::cook_and_run::CookAndRunCreate<'a> {
        crate::cook_and_run::CookAndRunCreate {
            id: cook_and_run_id,
            user_id: &self.user_id,
            name: &self.name,
            created: time,
            edited: time,
            occur: time,
        }
    }
}

impl AuthenticatedUser for CookAndRunCreateData {
    fn user_id(&self) -> AuthUser {
        AuthUser::Id(self.user_id.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookAndRun {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
    pub team_list: Vec<Team>,
    pub course_list: Vec<Course>,

    pub start_point: Option<Address>,
    pub end_point: Option<Address>,
    pub share_team_config: Option<ShareTeamConfig>,
    pub plan: Option<Plan>,
}

impl CookAndRun {
    pub fn from(cook_and_run: crate::cook_and_run::CookAndRun) -> Self {
        CookAndRun {
            id: cook_and_run.id,
            user_id: cook_and_run.user_id,
            name: cook_and_run.name,
            created: cook_and_run.created,
            edited: cook_and_run.edited,
            occur: cook_and_run.occur,
            team_list: cook_and_run.team_list.into_iter().map(Team::from).collect(),
            course_list: cook_and_run
                .course_list
                .into_iter()
                .map(Course::from)
                .collect(),
            start_point: cook_and_run.start_point.map(Address::from),
            end_point: cook_and_run.end_point.map(Address::from),
            share_team_config: cook_and_run.share_team_config.map(ShareTeamConfig::from),
            plan: cook_and_run.plan.map(Plan::from),
        }
    }
}

impl IntoResponse for CookAndRun {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl AuthenticatedUser for CookAndRun {
    fn user_id(&self) -> AuthUser {
        AuthUser::Id(self.user_id.clone())
    }
}

// Course models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCreateData {
    pub name: String,
    pub time: String, // HH:MM format
}

impl CourseCreateData {
    pub fn to(&self, cook_and_run_id: &Uuid, course_id: &Uuid) -> crate::course::Course {
        crate::course::Course {
            id: course_id.clone(),
            cook_and_run_id: cook_and_run_id.clone(),
            name: self.name.clone(),
            time: self.time.clone(),
            has_multiple_hosts: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseUpdateData {
    pub name: String,
    pub time: String,
    pub has_multiple_hosts: bool,
}
impl CourseUpdateData {
    pub fn to(&self, cook_and_run_id: &Uuid, course_id: &Uuid) -> crate::course::Course {
        crate::course::Course {
            id: course_id.clone(),
            cook_and_run_id: cook_and_run_id.clone(),
            name: self.name.clone(),
            time: self.time.clone(),
            has_multiple_hosts: self.has_multiple_hosts,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub time: String,
    pub has_multiple_hosts: bool,
}

impl Course {
    pub fn from(course: crate::course::Course) -> Self {
        Course {
            id: course.id,
            name: course.name,
            time: course.time,
            has_multiple_hosts: course.has_multiple_hosts,
        }
    }
}

impl IntoResponse for Course {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// Team models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamCreateData {
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub address: Address,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u32>,
    pub diets: Option<String>,
    pub needs_check: bool,
}

impl TeamCreateData {
    pub fn to(
        &self,
        cook_and_run_id: &Uuid,
        team_id: &Uuid,
        created_by_user: &str,
        time: &NaiveDateTime,
    ) -> crate::team::Team {
        let address = self.address.to();
        let team = crate::team::Team {
            id: team_id.clone(),
            cook_and_run_id: cook_and_run_id.clone(),
            created_by_user: Some(created_by_user.to_string()),
            name: self.name.clone(),
            created: *time,
            edited: *time,
            address: address,
            mail: self.mail.clone(),
            phone: self.phone.clone(),
            members: self.members,
            diets: self.diets.clone(),
            needs_check: self.needs_check,
            note_list: vec![],
        };
        team
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamUpdateData {
    pub name: String,
    pub address: Address,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u32>,
    pub diets: Option<String>,
    pub needs_check: bool,
}

impl TeamUpdateData {
    pub fn to(
        &self,
        cook_and_run_id: &Uuid,
        team_id: &Uuid,
        created_by_user: &str,
        time: &NaiveDateTime,
    ) -> crate::team::Team {
        let address = self.address.to();
        let team = crate::team::Team {
            id: team_id.clone(),
            cook_and_run_id: cook_and_run_id.clone(),
            created_by_user: Some(created_by_user.to_string()),
            name: self.name.clone(),
            created: *time,
            edited: *time,
            address: address,
            mail: self.mail.clone(),
            phone: self.phone.clone(),
            members: self.members,
            diets: self.diets.clone(),
            needs_check: self.needs_check,
            note_list: vec![],
        };
        team
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub created_by_user: String,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub address: Address,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u32>,
    pub diets: Option<String>,
    pub needs_check: bool,
    pub note_list: Vec<Note>,
}

impl IntoResponse for Team {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl Team {
    pub fn from(team: crate::team::Team) -> Self {
        Team {
            id: team.id,
            created_by_user: team.created_by_user.unwrap_or_default(),
            name: team.name,
            created: team.created,
            edited: team.edited,
            address: Address::from(team.address),
            mail: team.mail,
            phone: team.phone,
            members: team.members,
            diets: team.diets,
            needs_check: team.needs_check,
            note_list: team.note_list.into_iter().map(Note::from).collect(),
        }
    }
}

// Note models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteCreateData {
    pub headline: String,
    pub content: String,
}
impl NoteCreateData {
    pub(crate) fn to(&self, note_id: &Uuid, time: NaiveDateTime) -> crate::note::Note {
        crate::note::Note {
            id: note_id.clone(),
            headline: self.headline.clone(),
            content: self.content.clone(),
            created: time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub headline: String,
    pub content: String,
    pub created: NaiveDateTime,
}

impl Note {
    pub fn from(note: crate::note::Note) -> Self {
        Note {
            id: note.id,
            headline: note.headline,
            content: note.content,
            created: note.created,
        }
    }
}

impl IntoResponse for Note {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// Share Team Config models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareTeamConfig {
    pub id: Uuid,
    pub invite_text: String,
    pub needs_login: bool,
    pub default_needs_check: bool,
    pub required_fields: Vec<RequiredField>,
    pub max_teams: Option<u32>,
    pub registration_deadline: Option<NaiveDateTime>,
    pub created: NaiveDateTime,
}

impl ShareTeamConfig {
    pub fn from(config: crate::sharing::ShareTeamConfig) -> Self {
        ShareTeamConfig {
            id: config.id,
            invite_text: config.invite_text,
            needs_login: config.needs_login,
            default_needs_check: config.default_needs_check,
            required_fields: config
                .required_fields
                .into_iter()
                .map(RequiredField::from)
                .collect(),
            max_teams: config.max_teams,
            registration_deadline: config.registration_deadline,
            created: config.created,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredField {
    Mail,
    Phone,
    Members,
    Diets,
}

impl RequiredField {
    fn from(field: crate::sharing::RequiredField) -> Self {
        match field {
            crate::sharing::RequiredField::Mail => RequiredField::Mail,
            crate::sharing::RequiredField::Phone => RequiredField::Phone,
            crate::sharing::RequiredField::Members => RequiredField::Members,
            crate::sharing::RequiredField::Diets => RequiredField::Diets,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    Link,
    Account,
}

impl Access {
    fn from_list(db_access: Vec<crate::plan::Access>) -> Vec<Self> {
        db_access.into_iter().map(Access::from).collect()
    }
    fn from(db_access: crate::plan::Access) -> Self {
        match db_access {
            crate::plan::Access::Link => Access::Link,
            crate::plan::Access::Account => Access::Account,
        }
    }
}

// Plan models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub access: Vec<Access>,
    pub introduction: Option<String>,
    pub hosting_assignments: Vec<Hosting>,
    pub walking_paths: HashMap<Uuid, Vec<WalkingPathStep>>,
}

impl Plan {
    pub fn from(db_plan: crate::plan::Plan) -> Self {
        Plan {
            access: Access::from_list(db_plan.access),
            introduction: db_plan.introduction,
            hosting_assignments: db_plan
                .hosting_assignments
                .into_iter()
                .map(Hosting::from)
                .collect(),
            walking_paths: WalkingPathStep::from_map(db_plan.walking_paths),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hosting {
    pub id: Uuid,
    pub course_id: Uuid,
    pub team_id: Uuid,
    pub guest_team_ids: Vec<Uuid>,
}

impl Hosting {
    fn from(db_hosting: crate::plan::Hosting) -> Self {
        Hosting {
            id: db_hosting.id,
            course_id: db_hosting.course_id,
            team_id: db_hosting.team_id,
            guest_team_ids: db_hosting.guest_team_ids,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkingPathStep {
    pub course_id: Uuid,
    pub host_team_id: Uuid,
}

impl WalkingPathStep {
    fn from_map(
        db_steps: HashMap<Uuid, Vec<crate::plan::WalkingPathStep>>,
    ) -> HashMap<Uuid, Vec<Self>> {
        db_steps
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(WalkingPathStep::from).collect()))
            .collect()
    }

    fn from(db_step: crate::plan::WalkingPathStep) -> Self {
        WalkingPathStep {
            course_id: db_step.course_id,
            host_team_id: db_step.host_team_id,
        }
    }
}
