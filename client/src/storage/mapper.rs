use std::collections::HashMap;

use uuid::Uuid;

use super::{ContactData, CourseData, HostingData, PlanData};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Hosting {
    pub id: Uuid,
    pub course: CourseData,
    pub host: ContactData,
    pub guest_list: Vec<ContactData>,
}

impl Hosting {
    fn from_hosting_data(
        hosting_data: &HostingData,
        course_list: &Vec<CourseData>,
        contact_list: &Vec<ContactData>,
    ) -> Self {
        Hosting {
            id: hosting_data.id,
            course: find_course(hosting_data.name, course_list)
                .expect("Expect course")
                .clone(),
            host: find_contact(hosting_data.host, contact_list)
                .expect("Expect contact")
                .clone(),
            guest_list: hosting_data
                .guest_list
                .iter()
                .map(|&g| {
                    find_contact(g, contact_list)
                        .expect("Expect contact")
                        .clone()
                })
                .collect(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Plan {
    pub id: Uuid,
    pub hosting_list: Vec<Hosting>,
    pub walking_path: HashMap<ContactData, Vec<Hosting>>,
    pub greatest_distance: f64,
}

impl Plan {
    pub fn from_plan_data(
        plan_data: &PlanData,
        course_list: &Vec<CourseData>,
        contact_list: &Vec<ContactData>,
    ) -> Self {
        let id = plan_data.id;
        let hosting_list: Vec<Hosting> = plan_data
            .hosting_list
            .iter()
            .map(|h| Hosting::from_hosting_data(h, course_list, contact_list))
            .collect();
        let walking_path: HashMap<ContactData, Vec<Hosting>> = plan_data
            .walking_path
            .iter()
            .map(|(&contact_id, hosting_ids)| {
                let contact = find_contact(contact_id, contact_list)
                    .expect("Expect contact")
                    .clone();
                let hostings: Vec<Hosting> = hosting_ids
                    .iter()
                    .map(|&hosting_id| {
                        let host: Hosting = find_hosting(hosting_id, &hosting_list)
                            .expect("Expect hosting")
                            .clone();
                        host
                    })
                    .collect();
                (contact, hostings)
            })
            .collect();
        let greatest_distance = plan_data.greatest_distance;

        Plan {
            id,
            hosting_list,
            walking_path,
            greatest_distance,
        }
    }
}

fn find_contact(id: Uuid, contact_list: &Vec<ContactData>) -> Option<&ContactData> {
    for contact in contact_list.iter() {
        if contact.id == id {
            return Some(contact);
        }
    }
    None
}

fn find_course(id: Uuid, course_list: &Vec<CourseData>) -> Option<&CourseData> {
    for course in course_list.iter() {
        if course.id == id {
            return Some(course);
        }
    }
    None
}

fn find_hosting(id: Uuid, hosting_list: &Vec<Hosting>) -> Option<&Hosting> {
    for hosting in hosting_list.iter() {
        if hosting.id == id {
            return Some(hosting);
        }
    }
    None
}
