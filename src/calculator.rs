use std::{
    collections::{HashMap, HashSet},
    io::{Error, ErrorKind},
};

use crate::contact::{self, Contact};

#[derive(Debug)]
pub struct Calculator {
    start_point_latitude: f64,
    start_point_longitude: f64,
    goal_point_latitude: f64,
    goal_point_longitude: f64,
    course_name_list: Vec<String>,
    contact_list: Vec<Contact>,
}

struct Evolution {
    seed: String,
    group_list: Vec<Group>,
    score: u8,
}

struct Group {
    seed: f64,
    course_id: u8,
    host: Contact,
    guest_list: Vec<Contact>,
}

struct Course {
    name: String,
    host: Contact,
    guest_list: Vec<Contact>,
}

impl Calculator {
    pub fn calculate(&self) {
        //()-()
        //()-()
        //TODO
    }

    pub fn new(
        start_point_latitude_str: String,
        start_point_longitude_str: String,
        goal_point_latitude_str: String,
        goal_point_longitude_str: String,
        course_name_list: Vec<String>,
        contact_list: Vec<Contact>,
    ) -> Result<Self, Error> {
        let start_point_latitude = start_point_latitude_str.parse::<f64>().map_err(|err| {
            println!("Can't parse start_point_latitude string to f64 : {err}");
            return Error::new(
                ErrorKind::InvalidData,
                "Format of start point latitude is not a number!",
            );
        })?;

        let start_point_longitude = start_point_longitude_str.parse::<f64>().map_err(|err| {
            println!("Can't parse start_point_longitude string to f64 : {err}");
            return Error::new(
                ErrorKind::InvalidData,
                "Format of start point longitude is not a number!",
            );
        })?;

        let goal_point_latitude = goal_point_latitude_str.parse::<f64>().map_err(|err| {
            println!("Can't parse goal_point_latitude string to f64 : {err}");
            return Error::new(
                ErrorKind::InvalidData,
                "Format of start goal latitude is not a number!",
            );
        })?;

        let goal_point_longitude = goal_point_longitude_str.parse::<f64>().map_err(|err| {
            println!("Can't parse goal_point_longitude string to f64 : {err}");
            return Error::new(
                ErrorKind::InvalidData,
                "Format of start goal longitude is not a number!",
            );
        })?;

        Ok(Calculator {
            start_point_latitude,
            start_point_longitude,
            goal_point_latitude,
            goal_point_longitude,
            course_name_list,
            contact_list,
        })
    }

    fn seed_to_plan(&self, seed: &Vec<u8>) {
        let course_list = self.assign_courses(seed);
        self.assign_guests(seed, &course_list);
    }

    fn assign_guests(&self, seed: &Vec<u8>, course_list: &Vec<Course>) {
        //TODO hier weiter machen.
        let mut seen_contact_map_build = HashMap::new();
        for contact in &self.contact_list {
            let mut hash_set = &HashSet::new();
            seen_contact_map_build.insert(contact, hash_set);
        }
        let seen_contact_map = seen_contact_map_build;
        let course_map = course_list_to_map(course_list);
        let number_of_guests_per_course = self.contact_list.len() / self.course_name_list.len();

        let mut index = self.contact_list.len();
        let mut seed_len = seed.len();
        for (_, course_sub_list) in course_map.into_iter() {
            for course in course_sub_list {
                for index in 0..number_of_guests_per_course {
                    let guest = self.get_contact(
                        *seed.get(index % seed_len).unwrap(),
                        course,
                        &seen_contact_map,
                    );
                    let seen_guest_set = seen_contact_map.get(guest).unwrap();
                    seen_guest_set.insert(guest);
                }
            }
        }
    }

    fn get_contact(
        &self,
        mut seed: u8,
        course: &Course,
        seen_contact_map: &HashMap<&Contact, &HashSet<&Contact>>,
    ) -> &Contact {
        let mut contact;
        let contact_list_len = self.contact_list.len();
        loop {
            contact = self
                .contact_list
                .get(usize::from(seed) % contact_list_len)
                .unwrap();

            if contact.eq(&course.host) {
                seed += 1;
                continue;
            }

            let seen_contact_set = seen_contact_map.get(contact).unwrap();
            let mut seen = false;
            for guest in &course.guest_list {
                if seen_contact_set.contains(guest) {
                    seen = true;
                    break;
                }
            }

            if seen {
                seed += 1;
                continue;
            }
            break;
        }
        contact
    }

    fn assign_courses(&self, seed: &Vec<u8>) -> Vec<Course> {
        let mut contact_for_courses_list = self.contact_list.clone();
        let course_list_len = self.course_name_list.len();
        let mut course_list = Vec::new();

        let mut index = 0;
        loop {
            let seed_id = seed.get(index).unwrap();
            let course_name = self
                .course_name_list
                .get(index % course_list_len)
                .unwrap()
                .clone()
                .to_string();
            let contact_index = usize::from(*seed_id) % contact_for_courses_list.len();
            //TODO noch mal prüfen ob das Clonen sein muss
            let contact = contact_for_courses_list.get(contact_index).unwrap().clone();

            course_list.push(Course::new(course_name, contact));
            contact_for_courses_list.remove(contact_index);

            if contact_for_courses_list.is_empty() {
                break;
            }

            index += 1;
        }
        course_list
    }
}

impl Course {
    fn new(name: String, host: Contact) -> Self {
        Course {
            name,
            host,
            guest_list: Vec::new(),
        }
    }
}

fn calcDistance(
    start_point_latitude: f64,
    start_point_longitude: f64,
    goal_point_latitude: f64,
    goal_point_longitude: f64,
) -> f64 {
    f64::sqrt(
        (goal_point_latitude - start_point_latitude).powf(2_f64)
            + (goal_point_longitude - start_point_longitude).powf(2_f64),
    )
}

fn course_list_to_map(course_list: &Vec<Course>) -> HashMap<String, Vec<&Course>> {
    let mut course_map = HashMap::new();
    for course in course_list {
        let course_list = course_map.get(&course.name);
        if course_list.is_none() {
            course_map.insert(course.name.clone(), vec![course]);
        } else {
            course_list.unwrap().clone().push(course);
        }
    }
    course_map
}
