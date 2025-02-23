use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use rand::Rng;

use crate::contact::Contact;

#[derive(Debug)]
pub struct Calculator<'course_name_list, 'contact_list> {
    start_point_latitude: Option<i32>,
    start_point_longitude: Option<i32>,
    goal_point_latitude: Option<i32>,
    goal_point_longitude: Option<i32>,
    course_name_list: &'course_name_list Vec<String>,
    contact_list: &'contact_list Vec<Contact>,
    pub top_score: Arc<Mutex<TopScore>>,
}

#[derive(Debug)]
pub struct TopScore {
    pub score: Option<f64>,
    pub seed: Option<Vec<u8>>,
}

pub struct Plan<'course> {
    pub seed: Vec<u8>,
    pub course_map: HashMap<&'course String, Vec<CourseInternal>>,
    pub score: f64,
}

struct Course<'course> {
    name: &'course String,
    host: &'course Contact,
    guest_list: Vec<&'course Contact>,
}

struct CourseInternal {
    id: u8,
    host_id: u8,
    guest_id_list: Vec<u8>,
}

impl<'course_name_list, 'contact_list> Calculator<'course_name_list, 'contact_list> {
    pub fn new_with_start_and_goal(
        start_point_latitude: i32,
        start_point_longitude: i32,
        goal_point_latitude: i32,
        goal_point_longitude: i32,
        course_name_list: &'course_name_list Vec<String>,
        contact_list: &'contact_list Vec<Contact>,
    ) -> Self {
        let shared_data = Arc::new(Mutex::new(TopScore {
            score: None,
            seed: None,
        }));

        let thread_data = Arc::clone(&shared_data);
        Calculator {
            start_point_latitude: Some(start_point_latitude),
            start_point_longitude: Some(start_point_longitude),
            goal_point_latitude: Some(goal_point_latitude),
            goal_point_longitude: Some(goal_point_longitude),
            course_name_list,
            contact_list,
            top_score: thread_data,
        }
    }

    pub fn new(
        course_name_list: &'course_name_list Vec<String>,
        contact_list: &'contact_list Vec<Contact>,
    ) -> Self {
        let shared_data = Arc::new(Mutex::new(TopScore {
            score: None,
            seed: None,
        }));

        let thread_data = Arc::clone(&shared_data);
        Calculator {
            start_point_latitude: None,
            start_point_longitude: None,
            goal_point_latitude: None,
            goal_point_longitude: None,
            course_name_list,
            contact_list,
            top_score: thread_data,
        }
    }

    pub fn calculate(&self) {
        let number_of_seeds = 2;
        let mut list_of_seeds = Vec::new();
        for _ in 0..number_of_seeds {
            list_of_seeds.push(generate_seed());
        }

        //let pool = ThreadPool::new(5);

        for _ in 0..10 {
            let mut list_of_plans: Vec<Plan<'_>> = list_of_seeds
                .iter()
                .map(|seed| self.seed_to_plan(seed.clone()))
                .collect();
            list_of_plans.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

            let mut top_score = self.top_score.lock().unwrap();
            top_score.score = Some(list_of_plans[0].score);
            top_score.seed = Some(list_of_plans[0].seed.clone());

            list_of_seeds = generate_seed_from_plan_list(list_of_plans);
        }
    }

    fn seed_to_plan(&self, seed: Vec<u8>) -> Plan {
        let mut course_map = self.assign_courses(&seed);
        self.assign_guests(&seed, &mut course_map);

        let score = self.calc_score(
            self.start_point_latitude,
            self.start_point_longitude,
            self.goal_point_latitude,
            self.goal_point_longitude,
            &course_map,
        );

        Plan {
            course_map,
            score,
            seed,
        }
    }

    fn assign_guests(
        &self,
        seed: &Vec<u8>,
        course_map: &mut HashMap<&String, Vec<CourseInternal>>,
    ) {
        let mut seen_contact_map = HashMap::new();
        let mut seen_second_time_contact_map = HashMap::new();
        for contact in self.contact_list.iter() {
            seen_contact_map.insert(contact.id, HashSet::new());
            seen_second_time_contact_map.insert(contact.id, HashSet::new());
        }

        let number_of_guests_per_course = self.contact_list.len() / self.course_name_list.len();
        let mut index = self.contact_list.len();
        let seed_len = seed.len();

        for course_list in course_map.values_mut() {
            for course in course_list.iter_mut() {
                for _ in 0..number_of_guests_per_course {
                    let seed_value = *seed
                        .get(index % seed_len)
                        .expect("Expected seed value to find contact!");

                    let guest_optional = self
                        .get_contact(seed_value, course, &seen_contact_map)
                        .or_else(|| {
                            self.get_contact(seed_value, course, &seen_second_time_contact_map)
                        });

                    let guest = guest_optional
                        .expect("Expected to find guest that was not seen a second time!");

                    set_seen_people(&mut seen_contact_map, course, guest);
                    course.guest_id_list.push(guest.id);
                    index += 1;
                }
            }
        }
    }
    fn get_contact(
        &self,
        mut seed: u8,
        course: &CourseInternal,
        seen_contact_map: &HashMap<u8, HashSet<u8>>,
    ) -> Option<&Contact> {
        let contact_list_len = self.contact_list.len();
        for _ in 0..contact_list_len {
            let contact_index = usize::from(seed) % contact_list_len;
            let found_contact = self.contact_list.get(contact_index).expect(
                format!(
                    "Contact with index {} expected to find in contact list of length {}!",
                    contact_index, contact_list_len
                )
                .as_str(),
            );

            if found_contact.id == course.host_id {
                seed += 1;
                continue;
            }

            let seen_contact_set = seen_contact_map
                .get(&found_contact.id)
                .expect("Contact should be in seen contact map!");
            let mut seen = false;
            for guest in course.guest_id_list.iter() {
                if seen_contact_set.contains(guest) {
                    seen = true;
                    break;
                }
            }

            if seen {
                seed += 1;
                continue;
            } else {
                return Some(found_contact);
            }
        }
        None
    }

    fn assign_courses(&self, seed: &Vec<u8>) -> HashMap<&String, Vec<CourseInternal>> {
        let mut contact_for_courses_list = self
            .contact_list
            .iter()
            .enumerate()
            .map(|(_, contact)| contact)
            .collect::<Vec<&Contact>>();

        let mut course_map = HashMap::new();
        for course_name in self.course_name_list {
            course_map.insert(course_name, Vec::new());
        }

        let mut index = 0;
        loop {
            let seed_id = *seed
                .get(index % seed.len())
                .expect("Expected seed value to create course for course creation!");
            let course_index = index % self.course_name_list.len();
            let course_name = self
                .course_name_list
                .get(course_index)
                .expect("Expected to find course name for course creation!");
            let contact_index = usize::from(seed_id) % contact_for_courses_list.len();

            let contact = *contact_for_courses_list
                .get(contact_index)
                .expect("Expected contact for course creation!");

            course_map
                .get_mut(course_name)
                .expect("Expected course name in map for course creation!")
                .push(CourseInternal::new(
                    index
                        .try_into()
                        .expect("Expected number of courses i smaller then 255"),
                    contact.id,
                ));

            contact_for_courses_list.remove(contact_index);

            if contact_for_courses_list.is_empty() {
                break;
            }

            index += 1;
        }
        course_map
    }

    fn calc_score(
        &self,
        start_point_latitude: Option<i32>,
        start_point_longitude: Option<i32>,
        goal_point_latitude: Option<i32>,
        goal_point_longitude: Option<i32>,
        course_map: &HashMap<&String, Vec<CourseInternal>>,
    ) -> f64 {
        let contact_map = course_map_to_contact_map(course_map);
        let mut contact_walking_path = HashMap::new();

        for (_, course_list) in contact_map.iter() {
            for course in course_list {
                let path = contact_walking_path.entry(course.host_id).or_insert(vec![]);
                path.push(*course);
                for guest in course.guest_id_list.iter() {
                    let path = contact_walking_path.entry(*guest).or_insert(vec![]);
                    path.push(*course);
                }
            }
        }

        let mut distance = 0_f64;

        for (_, path) in contact_walking_path.iter() {
            if start_point_latitude.is_some() && start_point_longitude.is_some() {
                let first_course = path.get(0).expect("Expected first course in path!");
                let contact = self.contact_list.get(first_course.host_id as usize).expect(
                    format!(
                        "Expected contact with id {} in contact list!",
                        first_course.host_id
                    )
                    .as_str(),
                );

                distance += calc_distance(
                    start_point_latitude.expect("Expected start point latitude!"),
                    start_point_longitude.expect("Expected start point longitude!"),
                    contact.latitude,
                    contact.longitude,
                );
            }
            for i in 0..path.len() - 1 {
                let course_one = path.get(i).expect("Expected course one in path!");
                let course_two = path.get(i + 1).expect("Expected course two in path!");

                let contact_one = self.contact_list.get(course_one.host_id as usize).expect(
                    format!(
                        "Expected contact with id {} in contact list!",
                        course_one.host_id
                    )
                    .as_str(),
                );
                let contact_two = self.contact_list.get(course_two.host_id as usize).expect(
                    format!(
                        "Expected contact with id {} in contact list!",
                        course_two.host_id
                    )
                    .as_str(),
                );

                distance += calc_distance(
                    contact_one.latitude,
                    contact_one.longitude,
                    contact_two.latitude,
                    contact_two.longitude,
                );
            }
            if goal_point_latitude.is_some() && goal_point_longitude.is_some() {
                let last_course = path
                    .get(path.len() - 1)
                    .expect("Expected last course in path!");
                let contact = self.contact_list.get(last_course.host_id as usize).expect(
                    format!(
                        "Expected contact with id {} in contact list!",
                        last_course.host_id
                    )
                    .as_str(),
                );

                distance += calc_distance(
                    contact.latitude,
                    contact.longitude,
                    goal_point_latitude.expect("Expected goal point latitude!"),
                    goal_point_longitude.expect("Expected goal point longitude!"),
                );
            }
        }

        distance
    }

    pub fn top_plan(&self) -> Option<Plan> {
        let top_score = self.top_score.lock().unwrap().seed.clone();
        if top_score.is_none() {
            return None;
        }
        Some(self.seed_to_plan(top_score.unwrap()))
    }
}

impl CourseInternal {
    fn new(id: u8, host_id: u8) -> Self {
        CourseInternal {
            id,
            host_id,
            guest_id_list: Vec::new(),
        }
    }
}

fn calc_distance(
    start_point_latitude: i32,
    start_point_longitude: i32,
    goal_point_latitude: i32,
    goal_point_longitude: i32,
) -> f64 {
    f64::sqrt(
        ((goal_point_latitude - start_point_latitude).pow(2_u32)
            + (goal_point_longitude - start_point_longitude).pow(2_u32)) as f64,
    )
}

fn course_map_to_contact_map<'a>(
    course_map: &'a HashMap<&'a String, Vec<CourseInternal>>,
) -> HashMap<u8, Vec<&'a CourseInternal>> {
    let mut contact_map = HashMap::new();
    for course_list in course_map.values() {
        for course in course_list.iter() {
            contact_map
                .entry(course.host_id)
                .or_insert_with(Vec::new)
                .push(course);
            for guest_id in course.guest_id_list.iter() {
                contact_map
                    .entry(*guest_id)
                    .or_insert_with(Vec::new)
                    .push(course);
            }
        }
    }
    contact_map
}

fn set_seen_people<'contact, 'course: 'contact>(
    seen_contact_map: &mut HashMap<u8, HashSet<u8>>,
    course: &CourseInternal,
    new_guest: &'contact Contact,
) {
    {
        let seen_guest_set_guest = seen_contact_map
            .get_mut(&new_guest.id)
            .expect("Expected to find seen contact of new guest!");
        seen_guest_set_guest.insert(course.host_id);
    }
    {
        let seen_guest_set_host = seen_contact_map
            .get_mut(&course.host_id)
            .expect("Expected to find seen contact of host!");
        seen_guest_set_host.insert(new_guest.id);
    }

    for guest_id in course.guest_id_list.iter() {
        let seen_guest_set = seen_contact_map
            .get_mut(&new_guest.id)
            .expect("Expected to find seen contact of new guest!");
        seen_guest_set.insert(*guest_id);

        let seen_guest_set = seen_contact_map
            .get_mut(guest_id)
            .expect("Expected to find seen contact of guest!");
        seen_guest_set.insert(new_guest.id);
    }
}

fn generate_seed() -> Vec<u8> {
    let mut seed = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        seed.push(rng.gen());
    }
    seed
}

fn combine_seed(seed_one: &Vec<u8>, seed_two: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut rng = rand::thread_rng();
    let split_point = rng.gen_range(0..seed_one.len());

    let mut new_seed_one = seed_one[..split_point].to_vec();
    new_seed_one.extend_from_slice(&seed_two[split_point..]);

    let mut new_seed_two = seed_two[..split_point].to_vec();
    new_seed_two.extend_from_slice(&seed_one[split_point..]);

    (new_seed_one, new_seed_two)
}

fn generate_seed_from_plan_list(list_of_plans: Vec<Plan>) -> Vec<Vec<u8>> {
    let top_80_percent = (list_of_plans.len() as f64 * 0.8).ceil() as usize;
    let mut new_seeds = Vec::new();

    for i in (0..top_80_percent).step_by(2) {
        if i + 1 < top_80_percent {
            let (new_seed_one, new_seed_two) =
                combine_seed(&list_of_plans[i].seed, &list_of_plans[i + 1].seed);
            new_seeds.push(new_seed_one);
            new_seeds.push(new_seed_two);
        }
    }

    for _ in new_seeds.len()..list_of_plans.len() {
        new_seeds.push(generate_seed());
    }
    new_seeds
}
