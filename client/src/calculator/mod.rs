mod schema_calculator;
pub use schema_calculator::SchemaCalculator;

use std::collections::HashMap;

use chrono::NaiveTime;
use uuid::Uuid;

use crate::storage::{AddressData, CourseData, HostingData, PlanData, TeamData};

pub trait DataMapper<T> {
    fn from_data(data: &T) -> Self;
}

pub struct Plan {
    hosting_list: HashMap<Uuid /*Hosting ID */, Hosting>,
    walking_path: HashMap<Uuid /*Team ID */, Vec<Uuid /*Hosting ID */>>,
}

impl Plan {
    pub fn to_data(&self) -> PlanData {
        let hosting_list = self
            .hosting_list
            .iter()
            .map(|(id, hosting)| hosting.to_data(id))
            .collect();
        let walking_path = self
            .walking_path
            .iter()
            .map(|(team_id, hosting_ids)| (team_id.clone(), hosting_ids.clone()))
            .collect();
        PlanData {
            hosting_list,
            walking_path,
        }
    }
}

pub struct Hosting {
    pub course: Uuid, /*Course ID*/
    pub host: Uuid,   /*Team ID */
    pub guest_list: Vec<Uuid /*Team ID */>,
}

impl Hosting {
    fn to_data(&self, id: &Uuid) -> HostingData {
        HostingData {
            id: id.clone(),
            name: self.course,
            host: self.host,
            guest_list: self.guest_list.clone(),
        }
    }
}

pub struct Team {
    pub id: Uuid,
    pub point: Point,
}

impl DataMapper<TeamData> for Team {
    fn from_data(data: &TeamData) -> Self {
        Team {
            id: data.id,
            point: Point::from_data(&data.address),
        }
    }
}

pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl DataMapper<AddressData> for Point {
    fn from_data(data: &AddressData) -> Self {
        Point {
            latitude: data.latitude,
            longitude: data.longitude,
        }
    }
}

pub struct Course {
    pub id: Uuid,
    pub time: NaiveTime,
}

impl DataMapper<CourseData> for Course {
    fn from_data(data: &CourseData) -> Self {
        Course {
            id: data.id,
            time: data.time,
        }
    }
}

pub trait Calculator {
    fn calculate(&self) -> Plan;
}
