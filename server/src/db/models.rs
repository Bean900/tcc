use chrono::NaiveDateTime;
use diesel::{
    deserialize::FromSqlRow,
    expression::AsExpression,
    prelude::{Insertable, Queryable, Selectable},
};
use uuid::Uuid;

// ========================================
// Address
// ========================================
#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::db::schema::address)]
pub struct Address {
    pub id: Uuid,
    pub address_text: String,
    pub latitude: f64,
    pub longitude: f64,
}

// ========================================
// Team
// ========================================
#[derive(Queryable, Selectable, Insertable)]
#[diesel(belongs_to(CookAndRun))]
#[diesel(table_name = crate::db::schema::team)]
pub struct Team {
    pub id: Uuid,
    pub cook_and_run_id: Uuid,
    pub created_by_user: Option<String>,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub address: Uuid,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<i32>,
    pub diets: Option<String>,
    pub needs_check: bool,
}

// ========================================
// Note
// ========================================
#[derive(Queryable, Selectable, Insertable)]
#[diesel(belongs_to(Team))]
#[diesel(table_name = crate::db::schema::note)]
pub struct Note {
    pub id: Uuid,
    pub team_id: Uuid,
    pub headline: String,
    pub content: String,
    pub created: NaiveDateTime,
}

// ========================================
// Course
// ========================================
#[derive(Queryable, Selectable, Insertable)]
#[diesel(belongs_to(CookAndRun))]
#[diesel(table_name = crate::db::schema::course)]
pub struct Course {
    pub id: Uuid,
    pub cook_and_run_id: Uuid,
    pub name: String,
    pub time: String,
}

// ========================================
// Hosting
// ========================================
#[derive(Queryable, Selectable, Insertable)]
#[diesel(belongs_to(Plan))]
#[diesel(table_name = crate::db::schema::hosting)]
pub struct Hosting {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub course_id: Uuid,
    pub team_id: Uuid,
    pub guest_team_ids: serde_json::Value,
}

// ========================================
// Share
// ========================================
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::schema::sql_types::TeamFields)]
#[diesel(postgres_type(name = "team_fields"))]
pub enum TeamFields {
    Mail,
    Phone,
    Members,
    Diets,
}

impl<DB> diesel::deserialize::FromSql<crate::db::schema::sql_types::TeamFields, DB> for TeamFields
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        match s.as_str() {
            "Mail" => Ok(TeamFields::Mail),
            "Phone" => Ok(TeamFields::Phone),
            "Members" => Ok(TeamFields::Members),
            "Diets" => Ok(TeamFields::Diets),
            _ => Err(format!("Unknown variant: {}", s).into()),
        }
    }
}

impl<DB> diesel::serialize::ToSql<crate::db::schema::sql_types::TeamFields, DB> for TeamFields
where
    DB: diesel::backend::Backend,
    str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        let s = match self {
            TeamFields::Mail => "Mail",
            TeamFields::Phone => "Phone",
            TeamFields::Members => "Members",
            TeamFields::Diets => "Diets",
        };
        s.to_sql(out)
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(belongs_to(CookAndRun))]
#[diesel(table_name = crate::db::schema::share)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Share {
    pub id: Uuid,
    pub created: NaiveDateTime,
    pub invite_text: String,
    pub needs_login: bool,
    pub default_needs_check: bool,
    pub required_fields: Option<Vec<Option<TeamFields>>>,
    pub max_teams: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
}

// ========================================
// Plan
// ========================================
#[derive(Queryable, Selectable, Insertable)]
#[diesel(belongs_to(CookAndRun))]
#[diesel(table_name = crate::db::schema::plan)]
pub struct Plan {
    pub id: Uuid,
    pub access: serde_json::Value,
    pub introduction: Option<String>,
    pub walking_paths: serde_json::Value,
}

// ========================================
// CookAndRun
// ========================================
#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::cook_and_run)]
pub struct CookAndRunCreate<'a> {
    pub id: &'a Uuid,
    pub user_id: &'a str,
    pub name: &'a str,
    pub created: &'a NaiveDateTime,
    pub edited: &'a NaiveDateTime,
    pub occur: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::cook_and_run)]
pub struct CookAndRun {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
    pub course_with_multiple_hosts: Option<Uuid>,
    pub start_point: Option<Uuid>,
    pub end_point: Option<Uuid>,
    pub share_team_config: Option<Uuid>,
    pub plan: Option<Uuid>,
}
