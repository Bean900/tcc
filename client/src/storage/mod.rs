use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod local_storage;
pub mod mapper;
pub use local_storage::LocalStorage;

use std::f64::consts::PI;

pub trait StorageW {
    fn insert_auth_data(&mut self, auth_data: AuthData) -> Result<(), String>;

    fn create_cook_and_run_json(&mut self, uuid: Uuid, json: String) -> Result<(), String>;
    fn create_cook_and_run(&mut self, uuid: Uuid, name: String) -> Result<(), String>;
    fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String>;
    fn update_meta_of_cook_and_run(
        &mut self,
        id: Uuid,
        new_name: String,
        new_plan_text: Option<String>,
        occur: NaiveDate,
    ) -> Result<(), String>;
    fn add_team_to_cook_and_run(&mut self, id: Uuid, team: ContactData) -> Result<(), String>;
    fn update_team_in_cook_and_run(&mut self, id: Uuid, team: ContactData) -> Result<(), String>;
    fn create_team_note_in_cook_and_run(
        &mut self,
        id: Uuid,
        team_id: Uuid,
        headline: String,
        description: String,
    ) -> Result<(), String>;
    fn update_team_needs_ckeck_in_cook_and_run(
        &mut self,
        id: Uuid,
        team_id: Uuid,
        needs_check: bool,
    ) -> Result<(), String>;
    fn delete_team_in_cook_and_run(&mut self, id: Uuid, team_id: Uuid) -> Result<(), String>;
    fn update_start_point_in_cook_and_run(
        &mut self,
        id: Uuid,
        start_point: Option<MeetingPointData>,
    ) -> Result<(), String>;
    fn update_goal_point_in_cook_and_run(
        &mut self,
        id: Uuid,
        goal_point: Option<MeetingPointData>,
    ) -> Result<(), String>;
    fn add_course_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data: CourseData,
    ) -> Result<(), String>;
    fn update_course_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data: CourseData,
    ) -> Result<(), String>;
    fn delete_course_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data_id: Uuid,
    ) -> Result<(), String>;

    fn update_course_with_more_hosts_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data_id: Uuid,
    ) -> Result<(), String>;

    fn update_top_plan_in_cook_and_run(
        &mut self,
        id: Uuid,
        top_plan: Option<PlanData>,
    ) -> Result<(), String>;
}

pub trait StorageR {
    fn select_auth_data(&self) -> Result<AuthData, String>;

    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimalData>, String>;
    fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String>;
    fn select_cook_and_run_json(&self, id: Uuid) -> Result<String, String>; // Returns JSON string of CookAndRunData
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserData {
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthData {
    pub session_data: Option<SessionData>,
    pub process_data: Option<ProcessData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    pub access_token: String,
    pub id_token: String,
    pub user: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessData {
    pub code_verifier: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseData {
    pub id: Uuid,
    pub name: String,
    pub time: NaiveTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostingData {
    pub id: Uuid,
    pub name: Uuid, /*Course ID*/
    pub host: Uuid, /*Contact ID */
    pub guest_list: Vec<Uuid /*Contact ID */>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct ContactData {
    pub id: Uuid,
    pub team_name: String,
    pub address: AddressData,
    pub mail: String,
    pub phone_number: String,
    pub members: u32,
    pub diets: Vec<String>,
    pub needs_check: bool,
    pub notes: Vec<NoteData>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AddressData {
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl PartialEq for AddressData {
    fn eq(&self, other: &Self) -> bool {
        self.address.eq(&other.address)
    }
}

impl Eq for AddressData {}

impl Hash for AddressData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl AddressData {
    fn deg_to_rad(deg: f64) -> f64 {
        deg * PI / 180.0
    }

    pub fn distance(&self, addr: &AddressData) -> f64 {
        let r = 6371.0;

        let dlat = Self::deg_to_rad(addr.latitude - self.latitude);
        let dlon = Self::deg_to_rad(addr.longitude - self.longitude);

        let lat1_rad = Self::deg_to_rad(self.latitude);
        let lat2_rad = Self::deg_to_rad(addr.latitude);

        let a = (dlat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().asin();

        r * c
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct NoteData {
    pub id: Uuid,
    pub headline: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingPointData {
    pub name: String,
    pub time: NaiveTime,
    pub address: AddressData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanData {
    pub id: Uuid,
    pub hosting_list: Vec<HostingData>,
    pub walking_path: HashMap<Uuid /*Contact ID */, Vec<Uuid /*Hosting ID */>>,
    pub greatest_distance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookAndRunData {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub occur: NaiveDate,
    pub is_in_cloud: bool,
    pub contact_list: Vec<ContactData>,
    pub course_list: Vec<CourseData>,
    pub course_with_more_hosts: Option<Uuid>,
    pub start_point: Option<MeetingPointData>,
    pub end_point: Option<MeetingPointData>,
    pub top_plan: Option<PlanData>,
    pub plan_text: Option<String>,
    pub invite_allowed: bool,
    pub invite_text: Option<String>,
}

impl CookAndRunData {
    pub fn to_minimal(&self) -> CookAndRunMinimalData {
        CookAndRunMinimalData {
            id: self.id,
            name: self.name.clone(),
            created: self.created,
            edited: self.edited,
        }
    }
    pub fn new(id: Uuid, name: String) -> Self {
        CookAndRunData {
            id,
            name,
            created: Utc::now(),
            edited: Utc::now(),
            occur: Utc::now().date_naive(),
            is_in_cloud: false,
            contact_list: vec![],
            course_list: vec![],
            course_with_more_hosts: None,
            start_point: None,
            end_point: None,
            top_plan: None,
            plan_text: None,
            invite_allowed: false,
            invite_text: None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookAndRunMinimalData {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
}
