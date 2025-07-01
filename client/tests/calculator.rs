use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
};

use chrono::NaiveTime;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CourseData {
    pub id: Uuid,
    pub name: String,
    pub time: NaiveTime,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HostingData {
    pub id: Uuid,
    pub name: Uuid, /*Course ID*/
    pub host: Uuid, /*Contact ID */
    pub guest_list: Vec<Uuid /*Contact ID */>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContactData {
    pub id: Uuid,
    pub address: AddressData,
}

#[derive(Default, Debug, Clone)]
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MeetingPointData {
    pub name: String,
    pub time: NaiveTime,
    pub address: AddressData,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlanData {
    pub id: Uuid,
    pub hosting_list: Vec<HostingData>,
    pub walking_path: HashMap<Uuid /*Contact ID */, Vec<Uuid /*Hosting ID */>>,
    pub greatest_distance: f64,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CookAndRunData {
    pub id: Uuid,
    pub contact_list: Vec<ContactData>,
    pub course_list: Vec<CourseData>,
    pub course_with_more_hosts: Option<Uuid>,
    pub start_point: Option<MeetingPointData>,
    pub end_point: Option<MeetingPointData>,
}

pub struct Calculator {
    cook_and_run_data: CookAndRunData,
    top_plan: Arc<Mutex<Option<PlanData>>>,
}

impl Calculator {
    pub fn new(cook_and_run_data: CookAndRunData) -> Calculator {
        Calculator {
            cook_and_run_data,
            top_plan: Arc::new(Mutex::new(None)),
        }
    }

    pub fn calculate(&self) {
        todo!("Start async calculation")
    }

    pub fn stop(&self) {
        todo!("Stop async calculation")
    }

    pub fn get_top_plan(&self) -> Option<PlanData> {
        self.top_plan
            .lock()
            .expect("Expect to get lock on plan!")
            .clone()
    }
}
