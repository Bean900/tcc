mod schema_calculator;
pub use schema_calculator::SchemaCalculator;

use std::collections::HashMap;

use chrono::NaiveTime;
use uuid::Uuid;

pub struct Plan {
    hosting_list: HashMap<Uuid /*Hosting ID */, Hosting>,
    walking_path: HashMap<Uuid /*Team ID */, Vec<Uuid /*Hosting ID */>>,
}

pub struct Hosting {
    pub course: Uuid, /*Course ID*/
    pub host: Uuid,   /*Team ID */
    pub guest_list: Vec<Uuid /*Team ID */>,
}

pub struct Team {
    pub id: Uuid,
    pub point: Point,
}

pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct Course {
    pub id: Uuid,
    pub time: NaiveTime,
}

pub trait Calculator {
    fn calculate(&self) -> Plan;
}
