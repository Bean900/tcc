use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod local_storage;
pub mod mapper;
pub use local_storage::LocalStorage;

pub trait StorageW {
    fn create_cook_and_run(&mut self, uuid: Uuid, name: String) -> Result<(), String>;
    fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String>;
    fn rename_cook_and_run(&mut self, id: Uuid, new_name: String) -> Result<(), String>;
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
}

pub trait StorageR {
    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimalData>, String>;
    fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String>;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseData {
    pub id: Uuid,
    pub name: String,
    pub time: NaiveTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HostingData {
    id: Uuid,
    name: Uuid, /*Course ID*/
    host: Uuid, /*Contact ID */
    guest_list: Vec<Uuid /*Contact ID */>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct ContactData {
    pub id: Uuid,
    pub team_name: String,
    pub address: AddressData,
    pub mail: String,
    pub members: u32,
    pub allergies: Vec<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct NoteData {
    pub id: Uuid,
    pub headline: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingPointData {
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
    pub contact_list: Vec<ContactData>,
    pub course_list: Vec<CourseData>,
    pub course_with_more_hosts: Option<Uuid>,
    pub start_point: Option<MeetingPointData>,
    pub end_point: Option<MeetingPointData>,
    pub top_plan: Option<PlanData>,
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
            contact_list: vec![],
            course_list: vec![],
            course_with_more_hosts: None,
            start_point: None,
            end_point: None,
            top_plan: None,
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
