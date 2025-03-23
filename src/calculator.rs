use core::{hash, num};
use std::{
    collections::{hash_set, HashMap, HashSet},
    sync::{Arc, Mutex},
    thread::{self, spawn},
};

use log::{debug, info};

use rand::Rng;

use colored::Colorize;
use serde::de;
use threadpool::ThreadPool;

use crate::contact::Contact;

#[derive(Debug)]
pub struct Calculator {
    pub top_score: Arc<Mutex<TopScore>>,
    config: CalculatorConfig,
    calculating: Arc<Mutex<bool>>,
}

#[derive(Debug)]
pub struct CalculatorConfig {
    start_point_latitude: Option<i32>,
    start_point_longitude: Option<i32>,
    goal_point_latitude: Option<i32>,
    goal_point_longitude: Option<i32>,
    course_name_list: Vec<String>,
    course_with_more_hosts: Option<String>,
    contact_list: Vec<Contact>,
}

#[derive(Debug)]
pub struct TopScore {
    pub score: Option<f64>,
    pub seed: Option<Vec<u8>>,
}

impl TopScore {
    fn new() -> Self {
        TopScore {
            score: None,
            seed: None,
        }
    }
}

struct PlanInternal {
    seed: Vec<u8>,
    course_map: HashMap<String, Vec<CourseInternal>>,
    walking_path: HashMap<u8, HashSet<CourseInternal>>,
    score: f64,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct CourseInternal {
    id: u8,
    host_id: u8,
    guest_id_list: Vec<u8>,
}

//TODO where doesn't make much sense. I need to look into this
pub struct Plan<'course, 'contact>
where
    'contact: 'course,
    'course: 'contact,
{
    pub seed: Vec<u8>,
    pub course_map: HashMap<String, Vec<Course<'course>>>,
    pub walking_path: HashMap<&'contact Contact, Vec<Course<'course>>>,
    pub score: f64,
}

#[derive(Hash)]
pub struct Course<'contact> {
    pub name: String,
    pub host: &'contact Contact,
    pub guest_list: Vec<&'contact Contact>,
}

impl CalculatorConfig {
    pub fn new_with_start_and_goal(
        start_point_latitude: i32,
        start_point_longitude: i32,
        goal_point_latitude: i32,
        goal_point_longitude: i32,
        course_with_more_hosts: Option<String>,
        course_name_list: Vec<String>,
        contact_list: Vec<Contact>,
    ) -> Self {
        CalculatorConfig {
            start_point_latitude: Some(start_point_latitude),
            start_point_longitude: Some(start_point_longitude),
            goal_point_latitude: Some(goal_point_latitude),
            goal_point_longitude: Some(goal_point_longitude),
            course_with_more_hosts,
            course_name_list,
            contact_list,
        }
    }
    pub fn new(
        course_name_list: Vec<String>,
        contact_list: Vec<Contact>,
        course_with_more_hosts: Option<String>,
    ) -> Self {
        CalculatorConfig {
            start_point_latitude: None,
            start_point_longitude: None,
            goal_point_latitude: None,
            goal_point_longitude: None,
            course_with_more_hosts,
            course_name_list,
            contact_list,
        }
    }

    fn clone(&self) -> Self {
        CalculatorConfig {
            start_point_latitude: self.start_point_latitude.clone(),
            start_point_longitude: self.start_point_longitude.clone(),
            goal_point_latitude: self.goal_point_latitude.clone(),
            goal_point_longitude: self.goal_point_longitude.clone(),
            course_name_list: self.course_name_list.clone(),
            course_with_more_hosts: self.course_with_more_hosts.clone(),
            contact_list: self.contact_list.clone(),
        }
    }
}

impl Calculator {
    pub fn new(config: CalculatorConfig) -> Self {
        Calculator {
            config,
            top_score: Arc::new(Mutex::new(TopScore::new())),
            calculating: Arc::new(Mutex::new(true)),
        }
    }

    pub fn calculate(&self) {
        let number_of_threads = 5;
        *self
            .calculating
            .lock()
            .expect("Expect calculating to be set!") = true;
        for index in 0..number_of_threads {
            let config = self.config.clone();
            let top_score = Arc::clone(&self.top_score);
            let calculating = Arc::clone(&self.calculating);
            thread::spawn(move || {
                info!(
                    "Start calculation of thread {}/{}",
                    index + 1,
                    number_of_threads
                );
                calcutate_job(config, calculating, top_score);
                info!(
                    "Finished calculation of thread {}/{}",
                    index + 1,
                    number_of_threads
                );
            });
        }
    }

    pub fn stop(&self) {
        info!("Stop calculation");
        *self
            .calculating
            .lock()
            .expect("Expect calculating to be set!") = false;
    }

    pub fn top_plan(&self) -> Option<Plan> {
        let top_score_option = self.top_score.lock().unwrap().seed.clone();
        if top_score_option.is_none() {
            return None;
        }
        let top_score = top_score_option.expect("Score should be set!");
        let plan_internal = seed_to_plan(&self.config, top_score);
        let plan = Plan::new(&self.config.contact_list, plan_internal);
        Some(plan)
    }
}

fn calcutate_job(
    config: CalculatorConfig,
    calculating: Arc<Mutex<bool>>,
    top_score: Arc<Mutex<TopScore>>,
) {
    let number_of_seeds = 1_000;

    let mut list_of_seeds = Vec::new();
    for _ in 0..number_of_seeds {
        list_of_seeds.push(generate_seed());
    }

    loop {
        let mut list_of_plans: Vec<PlanInternal> = list_of_seeds
            .iter()
            .map(|seed| seed_to_plan(&config, seed.clone()))
            .collect();
        list_of_plans.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        if list_of_plans[0].score != f64::MAX {
            let mut top_score = top_score.lock().unwrap();
            if top_score.score.is_none() || list_of_plans[0].score < top_score.score.unwrap() {
                top_score.score = Some(list_of_plans[0].score);
                top_score.seed = Some(list_of_plans[0].seed.clone());
            }
        }
        if !*calculating.lock().expect("Expact to find calculating flag") {
            break;
        }
        list_of_seeds = generate_seed_from_plan_list(list_of_plans);
    }
}

fn seed_to_plan(config: &CalculatorConfig, seed: Vec<u8>) -> PlanInternal {
    let course_map = create_course_map(config, &seed);

    if course_map.is_none() {
        return PlanInternal {
            course_map: HashMap::new(),
            walking_path: HashMap::new(),
            score: f64::MAX,
            seed,
        };
    }
    let course_map = course_map.expect("Expected course map to be set!");
    let walking_path = calc_walking_path(&course_map);

    let score = calc_score(config, &walking_path);

    PlanInternal {
        course_map,
        walking_path,
        score,
        seed,
    }
}

fn create_course_map(
    config: &CalculatorConfig,
    seed: &Vec<u8>,
) -> Option<HashMap<String, Vec<CourseInternal>>> {
    let mut course_map = HashMap::new();
    let mut seen_contact_map = HashMap::new();
    let mut seen_contact_map_second_time = HashMap::new();

    for contact in config.contact_list.iter() {
        seen_contact_map.insert(contact, HashSet::new());
        seen_contact_map_second_time.insert(contact, HashSet::new());
    }

    let mut course_index = 0;
    let mut seed_index = 0;

    let mut possible_host_list = config.contact_list.iter().collect::<Vec<&Contact>>();
    for course_name in config.course_name_list.iter() {
        debug!("Calculating course \"{}\"", course_name);
        debug!(
            "List of people seen by each person:\n{:?}",
            seen_contact_map
                .iter()
                .map(|(k, v)| (
                    k.team_name.as_str(),
                    v.iter()
                        .map(|x: &&Contact| x.team_name.as_str())
                        .collect::<Vec<&str>>()
                ))
                .collect::<Vec<(&str, Vec<&str>)>>()
        );

        debug!(
            "List of second time people seen by each person:\n{:?}",
            seen_contact_map_second_time
                .iter()
                .map(|(k, v)| (
                    k.team_name.as_str(),
                    v.iter()
                        .map(|x: &&Contact| x.team_name.as_str())
                        .collect::<Vec<&str>>()
                ))
                .collect::<Vec<(&str, Vec<&str>)>>()
        );

        debug!(
            "All possible hosts:\t\t\t{:?}",
            possible_host_list
                .iter()
                .map(|x| x.team_name.as_str())
                .collect::<Vec<&str>>()
        );

        //Create list of possible hosts and guests, that will be used to create courses
        let mut possible_host_in_course_list = possible_host_list.clone();
        let mut possible_guest_list = config.contact_list.iter().collect::<Vec<&Contact>>();
        let mut contact_in_course: HashSet<&Contact> = HashSet::new();

        let number_of_courses = config.contact_list.len() / config.course_name_list.len()
            + if config.course_with_more_hosts.as_deref() == Some(course_name) {
                1
            } else {
                0
            };
        debug!("Number of courses for course: {}", number_of_courses);

        let number_of_guests_per_course = config.contact_list.len() / number_of_courses - 1;
        debug!(
            "Base number of guests per course: {}",
            number_of_guests_per_course
        );

        let mut number_of_guests_overhang = config.contact_list.len() % number_of_courses;
        debug!("Number of extra guests: {}", number_of_guests_overhang);
        for _ in 0..number_of_courses {
            debug!(
                "Possible hosts for course \"{}\":\t{:?}",
                course_name,
                possible_host_in_course_list
                    .iter()
                    .map(|x| x.team_name.as_str())
                    .collect::<Vec<&str>>()
            );
            debug!(
                "Possible guests in course \"{}\":\t{:?}",
                course_name,
                possible_guest_list
                    .iter()
                    .map(|x| x.team_name.as_str())
                    .collect::<Vec<&str>>()
            );

            let mut guest_list = Vec::new();

            //Choose host
            if possible_host_in_course_list.is_empty() {
                return None;
            }
            let host_index =
                seed[seed_index % seed.len()] as usize % possible_host_in_course_list.len();
            let host = *possible_host_in_course_list
                .get(host_index)
                .expect("Expected host to find in possible host list for this course!");
            //Remove host from possible host list in course
            possible_host_in_course_list.remove(host_index);

            //Remove host from possible host list
            let remove_host_index = possible_host_list
                .iter()
                .position(|x| *x == host)
                .expect("Expected host in list of possible hosts!");
            possible_host_list.remove(remove_host_index);

            //Remove host from possible guest list
            let remove_guest_index = possible_guest_list
                .iter()
                .position(|x| *x == host)
                .expect("Expected host in list of possible guests!");
            possible_guest_list.remove(remove_guest_index);

            seed_index += 1;

            set_seen_people(
                &mut seen_contact_map,
                &mut contact_in_course,
                &guest_list,
                host,
            );
            for _ in 0..(number_of_guests_per_course
                + if number_of_guests_overhang != 0 { 1 } else { 0 })
            {
                if possible_guest_list.is_empty() {
                    return None;
                }

                //Choose guest
                let guest = get_contact(
                    &possible_guest_list,
                    seed[seed_index % seed.len()],
                    &contact_in_course,
                    &seen_contact_map,
                )
                .or_else(|| {
                    get_contact(
                        &possible_guest_list,
                        seed[seed_index % seed.len()],
                        &contact_in_course,
                        &seen_contact_map_second_time,
                    )
                })
                .expect("Expected to find host for course!");

                //Remove guest from possible guest list
                let remove_guest_index = possible_guest_list
                    .iter()
                    .position(|x| *x == guest)
                    .expect("Expected guest in list of possible guests!");
                possible_guest_list.remove(remove_guest_index);

                //Remove guest from possible host list if exists
                let remove_host_index = possible_host_in_course_list
                    .iter()
                    .position(|x| *x == guest);
                if remove_host_index.is_some() {
                    possible_host_in_course_list.remove(
                        remove_host_index
                            .expect("Expected guest in list of possible hosts for this course!"),
                    );
                }

                seed_index += 1;

                set_seen_people(
                    &mut seen_contact_map,
                    &mut contact_in_course,
                    &guest_list,
                    host,
                );
                guest_list.push(guest);
            }
            if number_of_guests_overhang != 0 {
                number_of_guests_overhang -= 1;
            }
            debug!(
                "Create course: \n\tCourse:\t\"{}\"\n\tHost:\t\"{}\"\n\tGuests\t{:?}",
                course_name,
                host.team_name,
                guest_list
                    .iter()
                    .map(|x| x.team_name.as_str())
                    .collect::<Vec<&str>>()
            );

            let course = CourseInternal {
                id: course_index,
                host_id: host.id,
                guest_id_list: guest_list.iter().map(|guest| guest.id).collect(),
            };

            course_map
                .entry(course_name.clone())
                .or_insert_with(Vec::new)
                .push(course);

            course_index += 1;
        }
    }
    Some(course_map)
}

fn calc_score(
    config: &CalculatorConfig,
    contact_walking_path_set: &HashMap<u8, HashSet<CourseInternal>>,
) -> f64 {
    let mut longest_distance = 0_f64;

    for (_, path) in contact_walking_path_set.iter() {
        let mut path_iter = path.iter();
        let mut distance = 0_f64;
        let mut contact_from;
        let mut contact_to;

        let first_course = path_iter.next().expect("Expected first course in path!");
        contact_to = config
            .contact_list
            .get(first_course.host_id as usize)
            .expect(
                format!(
                    "Expected contact with id {} in contact list!",
                    first_course.host_id
                )
                .as_str(),
            );
        if config.start_point_latitude.is_some() && config.start_point_longitude.is_some() {
            distance += calc_distance(
                config
                    .start_point_latitude
                    .expect("Expected start point latitude!"),
                config
                    .start_point_longitude
                    .expect("Expected start point longitude!"),
                contact_to.latitude,
                contact_to.longitude,
            );
        }
        while let Some(course) = path_iter.next() {
            contact_from = contact_to;
            contact_to = config.contact_list.get(course.host_id as usize).expect(
                format!(
                    "Expected contact with id {} in contact list!",
                    course.host_id
                )
                .as_str(),
            );
            distance += calc_distance(
                contact_from.latitude,
                contact_from.longitude,
                contact_to.latitude,
                contact_to.longitude,
            );
        }

        if config.goal_point_latitude.is_some() && config.goal_point_longitude.is_some() {
            distance += calc_distance(
                contact_to.latitude,
                contact_to.longitude,
                config
                    .goal_point_latitude
                    .expect("Expected goal point latitude!"),
                config
                    .goal_point_longitude
                    .expect("Expected goal point longitude!"),
            );
        }
        if distance > longest_distance {
            longest_distance = distance;
        }
    }
    longest_distance
}

fn calc_walking_path(
    course_map: &HashMap<String, Vec<CourseInternal>>,
) -> HashMap<u8, HashSet<CourseInternal>> {
    let contact_map = course_map_to_contact_map(course_map);
    let mut contact_walking_path = HashMap::new();

    for (_, course_list) in contact_map.iter() {
        for &course in course_list {
            let path = contact_walking_path
                .entry(course.host_id)
                .or_insert(HashSet::new());
            path.insert((*course).clone());
            for guest in course.guest_id_list.iter() {
                let path = contact_walking_path.entry(*guest).or_insert(HashSet::new());
                path.insert((*course).clone());
            }
        }
    }

    contact_walking_path
}

fn get_contact<'contact>(
    possible_guest_list: &Vec<&'contact Contact>,
    seed_id: u8,
    contact_in_course: &HashSet<&'contact Contact>,
    seen_contact_map: &HashMap<&'contact Contact, HashSet<&'contact Contact>>,
) -> Option<&'contact Contact> {
    let mut seed = seed_id;
    for _ in 0..possible_guest_list.len() {
        let contact_index = usize::from(seed) % possible_guest_list.len();
        let contact = possible_guest_list
            .get(contact_index)
            .expect("Expected contact to find in contact list!");

        if contact_in_course.contains(contact) {
            seed = seed.wrapping_add(1);
            debug!(
                "Checking if \"{}\" could be guest... \t{}",
                contact.team_name,
                "Contact is already in a course!".red()
            );
            continue;
        }

        if !seen_contact_map
            .get(contact)
            .expect("Expected contact to find in seen contact map!")
            .is_disjoint(&contact_in_course)
        {
            seed = seed.wrapping_add(1);
            debug!(
                "Checking if \"{}\" could be guest... \t{}",
                contact.team_name,
                "Contact already seen other contact's!".red()
            );
            continue;
        }

        debug!(
            "Checking if \"{}\" could be guest... \t{}",
            contact.team_name,
            "Contact can be in course!".green()
        );
        return Some(contact);
    }
    None
}

impl CourseInternal {
    fn new(id: u8, host_id: u8) -> Self {
        CourseInternal {
            id,
            host_id,
            guest_id_list: Vec::new(),
        }
    }

    fn clone(&self) -> CourseInternal {
        CourseInternal {
            id: self.id,
            host_id: self.host_id,
            guest_id_list: self.guest_id_list.clone(),
        }
    }
}

impl<'contact> Course<'contact> {
    fn new(
        name: String,
        contact_list: &'contact Vec<Contact>,
        course_internal: &CourseInternal,
    ) -> Self {
        Course {
            name,
            host: &contact_list[course_internal.host_id as usize],
            guest_list: course_internal
                .guest_id_list
                .iter()
                .map(|guest_id| &contact_list[*guest_id as usize])
                .collect(),
        }
    }
}

impl<'contact, 'course> Plan<'contact, 'course> {
    fn new(contact_list: &'contact Vec<Contact>, plan: PlanInternal) -> Self {
        Plan {
            seed: plan.seed,
            course_map: plan
                .course_map
                .iter()
                .map(|(name, course_list)| {
                    (
                        name.clone(),
                        course_list
                            .iter()
                            .map(|course_internal| {
                                Course::new(name.clone(), contact_list, course_internal)
                            })
                            .collect(),
                    )
                })
                .collect(),
            walking_path: plan
                .walking_path
                .iter()
                .map(|(contact_id, course_list)| {
                    (
                        &contact_list[*contact_id as usize],
                        course_list
                            .iter()
                            .map(|course_internal| {
                                Course::new("".to_string(), contact_list, course_internal)
                            })
                            .collect(),
                    )
                })
                .collect(),
            score: plan.score,
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
    course_map: &'a HashMap<String, Vec<CourseInternal>>,
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

fn set_seen_people<'contact>(
    seen_contact_map: &mut HashMap<&'contact Contact, HashSet<&'contact Contact>>,
    contact_in_course: &mut HashSet<&'contact Contact>,
    guest_list: &Vec<&'contact Contact>,
    new_contact: &'contact Contact,
) {
    contact_in_course.insert(new_contact);
    guest_list.iter().for_each(|guest| {
        seen_contact_map
            .get_mut(new_contact)
            .expect("Expected to find seen contact of new contact!")
            .insert(guest);
        seen_contact_map
            .get_mut(guest)
            .expect("Expected to find seen contact of guest!")
            .insert(new_contact);
    });
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

fn generate_seed_from_plan_list(list_of_plans: Vec<PlanInternal>) -> Vec<Vec<u8>> {
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
