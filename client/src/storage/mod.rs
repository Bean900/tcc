use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod local_storage;
pub use local_storage::LocalStorage;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Course {
    id: Uuid,
    name: String,
    time: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Hosting {
    id: Uuid,
    name: Uuid, /*Course ID*/
    host: Uuid, /*Contact ID */
    guest_list: Vec<Uuid /*Contact ID */>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Contact {
    id: Uuid,
    team_name: String,
    address: String,
    latitude: f64,
    longitude: f64,
    allergies: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Plan {
    id: Uuid,
    hosting_list: Vec<Hosting>,
    walking_path: HashMap<Uuid /*Contact ID */, Vec<Uuid /*Hosting ID */>>,
    greatest_distance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookAndRun {
    id: Uuid,
    name: String,
    created: DateTime<Utc>,
    edited: DateTime<Utc>,
    contact_list: Vec<Contact>,
    course_list: Vec<Course>,
    top_plan: Option<Plan>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookAndRunMinimal {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
}

impl CookAndRun {
    fn to_minimal(&self) -> CookAndRunMinimal {
        CookAndRunMinimal {
            id: self.id,
            name: self.name.clone(),
            created: self.created,
            edited: self.edited,
        }
    }
    fn new(name: String) -> Self {
        CookAndRun {
            id: Uuid::new_v4(),
            name,
            created: Utc::now(),
            edited: Utc::now(),
            contact_list: vec![],
            course_list: vec![],
            top_plan: None,
        }
    }
}

pub trait Storage {
    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimal>, String>;
    fn create_cook_and_run(&mut self, name: String) -> Result<Uuid, String>;
}
