use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod local_storage;
mod mapper;
pub use local_storage::LocalStorage;

pub trait StorageW {
    fn create_cook_and_run(&mut self, uuid: Uuid, name: String) -> Result<(), String>;
    fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String>;
    fn rename_cook_and_run(&mut self, id: Uuid, new_name: String) -> Result<(), String>;
    fn add_team_to_cook_and_run(&mut self, id: Uuid, team: ContactData) -> Result<(), String>;
    fn update_team_in_cook_and_run(&mut self, id: Uuid, team: ContactData) -> Result<(), String>;
}

pub trait StorageR {
    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimalData>, String>;
    fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String>;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CourseData {
    id: Uuid,
    name: String,
    time: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HostingData {
    id: Uuid,
    name: Uuid, /*Course ID*/
    host: Uuid, /*Contact ID */
    guest_list: Vec<Uuid /*Contact ID */>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactData {
    pub id: Uuid,
    pub team_name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub mail: String,
    pub members: u32,
    pub allergies: Vec<String>,
    pub needs_check: bool,
    pub notes: Vec<NoteData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteData {
    pub id: Uuid,
    pub headline: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PlanData {
    id: Uuid,
    hosting_list: Vec<HostingData>,
    walking_path: HashMap<Uuid /*Contact ID */, Vec<Uuid /*Hosting ID */>>,
    greatest_distance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookAndRunData {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub contact_list: Vec<ContactData>,
    pub course_list: Vec<CourseData>,
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
