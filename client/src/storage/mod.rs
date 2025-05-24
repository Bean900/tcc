use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod local_storage;
mod mapper;
pub use local_storage::LocalStorage;

pub trait StorageW {
    fn create_cook_and_run(&mut self, uuid: Uuid, name: String) -> Result<(), String>;
}

pub trait StorageR {
    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimalData>, String>;
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
struct ContactData {
    id: Uuid,
    team_name: String,
    address: String,
    latitude: f64,
    longitude: f64,
    allergies: Vec<String>,
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
    name: String,
    created: DateTime<Utc>,
    edited: DateTime<Utc>,
    contact_list: Vec<ContactData>,
    course_list: Vec<CourseData>,
    top_plan: Option<PlanData>,
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
