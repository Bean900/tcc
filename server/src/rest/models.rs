use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    plan,
    rest::auth::{AuthUser, AuthenticatedUser},
};

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

impl IntoResponse for Address {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// Point model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub address: Address,
    pub name: String,
    pub time: String,
}

impl Point {
    pub fn from(point: crate::point::Point) -> Self {
        Point {
            address: Address::from(point.address),
            name: point.name,
            time: point.time,
        }
    }

    pub fn to(&self) -> crate::point::Point {
        crate::point::Point {
            id: Uuid::new_v4(),
            address: self.address.to(),
            name: self.name.clone(),
            time: self.time.clone(),
        }
    }
}

impl IntoResponse for Point {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
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

impl IntoResponse for CookAndRunMeta {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
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

    pub start_point: Option<Point>,
    pub end_point: Option<Point>,
    pub share_team_config: Option<ShareTeamConfig>,
    pub plan: Option<Plan>,
    pub plan_config: Option<PlanConfig>,
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
            start_point: cook_and_run.start_point.map(Point::from),
            end_point: cook_and_run.end_point.map(Point::from),
            share_team_config: cook_and_run.share_team_config.map(ShareTeamConfig::from),
            plan: cook_and_run.plan.map(Plan::from),
            plan_config: cook_and_run.plan_config.map(PlanConfig::from),
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
    pub user_id: Option<String>,
    pub address: Address,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u32>,
    pub diets: Option<String>,
    #[serde(default)]
    pub needs_check: bool,
}

impl TeamCreateData {
    pub fn to(
        &self,
        cook_and_run_id: &Uuid,
        team_id: &Uuid,
        time: &NaiveDateTime,
    ) -> crate::team::Team {
        let address = self.address.to();
        let team = crate::team::Team {
            id: team_id.clone(),
            cook_and_run_id: cook_and_run_id.clone(),
            created_by_user: self.user_id.clone(),
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

impl AuthenticatedUser for TeamCreateData {
    fn user_id(&self) -> AuthUser {
        if let Some(user_id) = &self.user_id {
            AuthUser::Id(user_id.clone())
        } else {
            AuthUser::Anonymous
        }
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

impl IntoResponse for ShareTeamConfig {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
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

// Plan models
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    Link,
    Account,
}

impl Access {
    fn from(field: plan::Access) -> Self {
        match field {
            plan::Access::Link => Access::Link,
            plan::Access::Account => Access::Account,
        }
    }

    fn from_list(db_field_list: Vec<plan::Access>) -> Vec<Self> {
        db_field_list.into_iter().map(Access::from).collect()
    }

    fn to(&self) -> plan::Access {
        match self {
            Access::Link => plan::Access::Link,
            Access::Account => plan::Access::Account,
        }
    }

    fn to_list(access_list: &[Access]) -> Vec<plan::Access> {
        access_list.iter().map(Access::to).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Deu,
    Eng,
}

impl Language {
    fn from(field: plan::Language) -> Self {
        match field {
            plan::Language::DEUTSCH => Language::Deu,
            plan::Language::ENGLISH => Language::Eng,
        }
    }

    fn to(&self) -> plan::Language {
        match self {
            Language::Deu => plan::Language::DEUTSCH,
            Language::Eng => plan::Language::ENGLISH,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanConfig {
    access: Vec<Access>,
    title: String,
    description: String,
    date: NaiveDate,
    language: Language,
}

impl PlanConfig {
    pub fn from(plan_config: plan::PlanConfig) -> Self {
        PlanConfig {
            access: Access::from_list(plan_config.access),
            title: plan_config.title,
            description: plan_config.description,
            date: plan_config.date,
            language: Language::from(plan_config.language),
        }
    }

    pub fn to(&self) -> plan::PlanConfig {
        plan::PlanConfig {
            access: Access::to_list(&self.access),
            title: self.title.clone(),
            description: self.description.clone(),
            date: self.date,
            language: self.language.to(),
        }
    }
}

impl IntoResponse for PlanConfig {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hosting {
    pub id: Uuid,
    pub name: Uuid,            // Course ID
    pub host: Uuid,            // Team ID
    pub guest_list: Vec<Uuid>, // Team ID
}

impl Hosting {
    pub fn from(db_hosting: plan::Hosting) -> Self {
        Hosting {
            id: db_hosting.id,
            name: db_hosting.name,
            host: db_hosting.host,
            guest_list: db_hosting.guest_list,
        }
    }

    pub fn to(&self) -> plan::Hosting {
        plan::Hosting {
            id: self.id,
            name: self.name,
            host: self.host,
            guest_list: self.guest_list.clone(),
        }
    }
}

impl IntoResponse for Hosting {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub hosting_list: Vec<Hosting>,
    pub walking_path: HashMap<Uuid, Vec<Uuid>>,
}

impl Plan {
    pub fn from(plan: plan::Plan) -> Self {
        Plan {
            hosting_list: plan.hosting_list.into_iter().map(Hosting::from).collect(),
            walking_path: plan.walking_path.clone(),
        }
    }

    pub fn to(&self) -> plan::Plan {
        plan::Plan {
            hosting_list: self.hosting_list.iter().map(Hosting::to).collect(),
            walking_path: self.walking_path.clone(),
        }
    }
}

impl IntoResponse for Plan {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
