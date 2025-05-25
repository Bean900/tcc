use std::{collections::HashMap, rc::Rc};

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Course {
    id: Uuid,
    pub name: String,
    pub time: DateTime<Utc>,
}

pub struct Hosting {
    id: Uuid,
    pub name: Rc<Course>,
    pub host: Rc<Contact>,
    pub guest_list: Vec<Rc<Contact>>,
}

pub struct Contact {
    pub id: Uuid,
    pub team_name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub allergies: Vec<String>,
}

pub struct Plan {
    id: Uuid,
    pub walking_path: HashMap<Rc<Contact>, Vec<Rc<Hosting>>>,
    pub greatest_distance: f64,
}

pub struct CookAndRun {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub contact_list: Vec<Rc<Contact>>,
    pub course_list: Vec<Rc<Course>>,
    pub top_plan: Option<Plan>,
}
