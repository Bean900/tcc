use crate::contact::Contact;
use std::ptr::null;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use rand::Rng;

#[derive(Debug)]
pub struct Calculator {
    start_point_latitude: i32,
    start_point_longitude: i32,
    goal_point_latitude: i32,
    goal_point_longitude: i32,
    course_name_list: Vec<String>,
    contact_list: Vec<Contact>,
    pub top_score: Arc<Mutex<TopScore>>,
}

#[derive(Debug)]
pub struct TopScore {
    pub score: Option<f64>,
    pub seed: Option<Vec<u8>>,
}

struct Plan<'calc> {
    seed: Vec<u8>,
    course_list: Vec<Course<'calc>>,
    score: f64,
}

struct Course<'calc> {
    name: &'calc String,
    host: &'calc Contact,
    guest_list: Vec<&'calc Contact>,
}

impl Calculator {
    pub fn new(
        start_point_latitude: i32,
        start_point_longitude: i32,
        goal_point_latitude: i32,
        goal_point_longitude: i32,
        course_name_list: Vec<String>,
        contact_list: Vec<Contact>,
    ) -> Self {
        let shared_data = Arc::new(Mutex::new(TopScore {
            score: None,
            seed: None,
        }));

        let thread_data = Arc::clone(&shared_data);
        Calculator {
            start_point_latitude,
            start_point_longitude,
            goal_point_latitude,
            goal_point_longitude,
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

        loop {
            println!("SEED: {:?}", list_of_seeds);
            let mut list_of_plans: Vec<Plan<'_>> = list_of_seeds
                .iter()
                .map(|seed| self.seed_to_plan(seed.clone()))
                .collect();
            list_of_plans.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

            println!(
                "Score of plans: {:?}",
                list_of_plans
                    .iter()
                    .map(|plan| plan.score)
                    .collect::<Vec<f64>>()
            );

            let mut top_score = self.top_score.lock().unwrap();
            top_score.score = Some(list_of_plans[0].score);
            top_score.seed = Some(list_of_plans[0].seed.clone());

            list_of_seeds = generate_seed_from_plan_list(list_of_plans);
        }
    }

    fn seed_to_plan(&self, seed: Vec<u8>) -> Plan {
        let mut course_list = self.assign_courses(&seed);
        self.assign_guests(&seed, &mut course_list);

        self.assign_guests(&seed, &course_list);

        let score = calc_score(
            self.start_point_latitude,
            self.start_point_longitude,
            self.goal_point_latitude,
            self.goal_point_longitude,
            &course_list,
        );

        Plan {
            course_list,
            score,
            seed,
        }
    }

    fn assign_guests<'a>(&'a self, seed: &Vec<u8>, course_list: &'a mut Vec<Course<'a>>) {
        let mut seen_contact_map: HashMap<&Contact, HashSet<&Contact>> = HashMap::new();
        let mut seen_second_time_contact_map: HashMap<&Contact, HashSet<&Contact>> = HashMap::new();
        for contact in &self.contact_list {
            seen_contact_map.insert(contact, HashSet::new());
            seen_second_time_contact_map.insert(contact, HashSet::new());
        }

        let mut course_map = course_list_to_map_mut(course_list);

        let number_of_guests_per_course = self.contact_list.len() / self.course_name_list.len();
        let mut index = self.contact_list.len();
        let seed_len = seed.len();

        for (_, course_sub_list) in course_map.iter_mut() {
            for course in course_sub_list {
                for _ in 0..number_of_guests_per_course {
                    let mut guest = self.get_contact(
                        *seed.get(index % seed_len).unwrap(),
                        course,
                        &seen_contact_map,
                    );

                    if guest.is_none() {
                        guest = self.get_contact(
                            *seed.get(index % seed_len).unwrap(),
                            course,
                            &seen_second_time_contact_map,
                        );
                        if guest.is_none() {
                            log::error!("No Guest found.")
                        }
                    }

                    set_seen_people(&mut seen_contact_map, course, guest.unwrap());
                    course.guest_list.push(guest.unwrap());
                    index += 1;
                }
            }
        }
    }

    fn get_contact(
        &self,
        mut seed: u8,
        course: &Course,
        seen_contact_map: &HashMap<&Contact, HashSet<&Contact>>,
    ) -> Option<&Contact> {
        let contact_list_len = self.contact_list.len();
        for _ in 0..contact_list_len {
            let found_contact = self
                .contact_list
                .get(usize::from(seed) % contact_list_len)
                .unwrap();

            if found_contact.eq(&course.host) {
                seed += 1;
                continue;
            }

            let seen_contact_set = seen_contact_map.get(found_contact).unwrap();
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
            } else {
                return Some(found_contact);
            }
        }
        None
    }

    fn assign_courses(&self, seed: &Vec<u8>) -> Vec<Course> {
        let mut contact_for_courses_list: Vec<&Contact> = self.contact_list.iter().collect();
        let course_list_len = self.course_name_list.len();
        let mut course_list = Vec::new();

        let mut index = 0;
        loop {
            let seed_id = *seed.get(index).unwrap();
            let course_name = self.course_name_list.get(index % course_list_len).unwrap();
            let contact_index = usize::from(seed_id) % contact_for_courses_list.len();

            {
                let contact = *contact_for_courses_list.get(contact_index).unwrap();
                course_list.push(Course::new(course_name, contact));
            }

            contact_for_courses_list.remove(contact_index);

            if contact_for_courses_list.is_empty() {
                break;
            }

            index += 1;
        }
        course_list
    }
}

impl<'calc> Course<'calc> {
    fn new(name: &'calc String, host: &'calc Contact) -> Self {
        Course {
            name,
            host,
            guest_list: Vec::new(),
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

fn calc_score(
    start_point_latitude: i32,
    start_point_longitude: i32,
    goal_point_latitude: i32,
    goal_point_longitude: i32,
    course_list: &Vec<Course>,
) -> f64 {
    let contact_map = course_list_to_map(course_list);
    let mut contact_walking_path: HashMap<&Contact, Vec<&Contact>> = HashMap::new();

    log::warn!("Number of courses: {}", course_list.len());

    for (_, course_list) in contact_map.iter() {
        for &course in course_list {
            log::warn!("Course name: {:?}", course.name);
            log::warn!("Host: {:?}", course.host.team_name);
            log::warn!(
                "Guests: {:?}",
                course
                    .guest_list
                    .iter()
                    .map(|contact| contact.team_name.clone())
                    .collect::<Vec<String>>()
            );
            let path = contact_walking_path.entry(course.host).or_insert(vec![]);
            path.push(course.host);
            let guest_list = &course.guest_list;
            for guest in guest_list {
                let path = contact_walking_path.entry(guest).or_insert(vec![]);
                path.push(course.host);
            }
        }
    }
    log::warn!("Number of walking paths: {}", contact_walking_path.len());

    let mut distance = 0_f64;

    for (_, path) in contact_walking_path.iter() {
        log::warn!(
            "Path with all team names: {:?}",
            path.iter()
                .map(|contact| contact.team_name.clone())
                .collect::<Vec<String>>()
        );
        for i in 0..path.len() - 1 {
            let contact_one = path.get(i).unwrap();
            let contact_two = path.get(i + 1).unwrap();

            distance += calc_distance(
                contact_one.latitude,
                contact_one.longitude,
                contact_two.latitude,
                contact_two.longitude,
            );
        }
    }

    distance
}

fn course_list_to_map_mut<'a>(
    course_list: &'a mut Vec<Course<'a>>,
) -> HashMap<&'a String, Vec<&'a mut Course<'a>>> {
    let mut course_map = HashMap::new();
    for course in course_list.iter_mut() {
        let course_list_opt = course_map.get_mut(course.name);
        if course_list_opt.is_none() {
            course_map.insert(course.name, vec![course]);
        } else {
            let course_list = course_list_opt.unwrap();
            course_list.push(course);
        }
    }
    course_map
}

fn course_list_to_map<'a>(
    course_list: &'a Vec<Course<'a>>,
) -> HashMap<&'a String, Vec<&'a Course<'a>>> {
    let mut course_map = HashMap::new();
    for course in course_list.iter() {
        let course_list_opt = course_map.get_mut(course.name);
        if course_list_opt.is_none() {
            course_map.insert(course.name, vec![course]);
        } else {
            let course_list = course_list_opt.unwrap();
            course_list.push(course);
        }
    }
    course_map
}

fn set_seen_people<'calc>(
    seen_contact_map: &mut HashMap<&Contact, HashSet<&'calc Contact>>,
    course: &Course<'calc>,
    new_guest: &'calc Contact,
) {
    {
        let seen_guest_set_guest = seen_contact_map.get_mut(new_guest).unwrap();
        seen_guest_set_guest.insert(course.host);
    }
    {
        let seen_guest_set_host = seen_contact_map.get_mut(course.host).unwrap();
        seen_guest_set_host.insert(new_guest);
    }

    for &guest in &course.guest_list {
        let seen_guest_set = seen_contact_map.get_mut(new_guest).unwrap();
        seen_guest_set.insert(&guest);

        let seen_guest_set = seen_contact_map.get_mut(guest).unwrap();
        seen_guest_set.insert(new_guest);
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
