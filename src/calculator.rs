use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use rand::Rng;

use crate::contact::Contact;

#[derive(Debug)]
pub struct Calculator {
    start_point_latitude: Option<i32>,
    start_point_longitude: Option<i32>,
    goal_point_latitude: Option<i32>,
    goal_point_longitude: Option<i32>,
    course_name_list: Vec<String>,
    contact_list: Vec<Contact>,
    pub top_score: Arc<Mutex<TopScore>>,
}

#[derive(Debug)]
pub struct TopScore {
    pub score: Option<f64>,
    pub seed: Option<Vec<u8>>,
}

pub struct Plan<'calc> {
    pub seed: Vec<u8>,
    pub course_list: Vec<Course<'calc>>,
    pub score: f64,
}

struct Course<'calc> {
    name: &'calc String,
    host_index: usize,
    guest_index_list: Vec<usize>,
}

impl Calculator {
    pub fn new_with_start_and_goal(
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
            start_point_latitude: Some(start_point_latitude),
            start_point_longitude: Some(start_point_longitude),
            goal_point_latitude: Some(goal_point_latitude),
            goal_point_longitude: Some(goal_point_longitude),
            course_name_list,
            contact_list,
            top_score: thread_data,
        }
    }

    pub fn new(course_name_list: Vec<String>, contact_list: Vec<Contact>) -> Self {
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

        for i in 0..10 {
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

        let score = self.calc_score(
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

    fn assign_guests(&self, seed: &Vec<u8>, course_list: &mut Vec<Course>) {
        let mut seen_contact_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut seen_second_time_contact_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (index, _) in self.contact_list.iter().enumerate() {
            seen_contact_map.insert(index, HashSet::new());
            seen_second_time_contact_map.insert(index, HashSet::new());
        }

        let course_map = course_list_to_map(course_list);

        let number_of_guests_per_course = self.contact_list.len() / self.course_name_list.len();
        let mut index = self.contact_list.len();
        let seed_len = seed.len();

        for (_, course_sub_list) in course_map.iter() {
            for course_index in course_sub_list {
                let course = &mut course_list[*course_index];
                for _ in 0..number_of_guests_per_course {
                    let guest = self
                        .get_contact(
                            *seed.get(index % seed_len).unwrap(),
                            course,
                            &seen_contact_map,
                        )
                        .or_else(|| {
                            self.get_contact(
                                *seed.get(index % seed_len).unwrap(),
                                course,
                                &seen_second_time_contact_map,
                            )
                        });

                    if guest.is_none() {
                        log::error!("No Guest found.")
                    }

                    set_seen_people(&mut seen_contact_map, course, guest.unwrap());
                    course.guest_index_list.push(guest.unwrap());
                    index += 1;
                }
            }
        }
    }
    fn get_contact(
        &self,
        mut seed: u8,
        course: &Course,
        seen_contact_map: &HashMap<usize, HashSet<usize>>,
    ) -> Option<usize> {
        let contact_list_len = self.contact_list.len();
        for _ in 0..contact_list_len {
            let found_contact_index = (usize::from(seed) % contact_list_len);

            if found_contact_index == course.host_index {
                seed += 1;
                continue;
            }

            let seen_contact_set = seen_contact_map.get(&found_contact_index).unwrap();
            let mut seen = false;
            for guest in &course.guest_index_list {
                if seen_contact_set.contains(guest) {
                    seen = true;
                    break;
                }
            }

            if seen {
                seed += 1;
                continue;
            } else {
                return Some(found_contact_index);
            }
        }
        None
    }

    fn assign_courses(&self, seed: &Vec<u8>) -> Vec<Course> {
        let mut contact_for_courses_list: Vec<usize> = (0..self.contact_list.len()).collect();
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

    fn calc_score(
        &self,
        start_point_latitude: Option<i32>,
        start_point_longitude: Option<i32>,
        goal_point_latitude: Option<i32>,
        goal_point_longitude: Option<i32>,
        course_list: &Vec<Course>,
    ) -> f64 {
        let contact_map = course_list_to_map(course_list);
        let mut contact_walking_path: HashMap<usize, Vec<usize>> = HashMap::new();

        log::warn!("Number of courses: {}", course_list.len());

        for (_, course_list_from_vec) in contact_map.iter() {
            for &course_index in course_list_from_vec {
                let course = &course_list[course_index];
                log::warn!("Course name: {:?}", course.name);
                log::warn!("Host: {:?}", self.contact_list[course.host_index].team_name);
                log::warn!(
                    "Guests: {:?}",
                    course
                        .guest_index_list
                        .iter()
                        .map(|contact| self.contact_list[*contact].team_name.clone())
                        .collect::<Vec<String>>()
                );
                let path = contact_walking_path
                    .entry(course.host_index)
                    .or_insert(vec![]);
                path.push(course.host_index);
                let guest_list = &course.guest_index_list;
                for guest in guest_list {
                    let path = contact_walking_path.entry(*guest).or_insert(vec![]);
                    path.push(course.host_index);
                }
            }
        }
        log::warn!("Number of walking paths: {}", contact_walking_path.len());

        let mut distance = 0_f64;

        for (_, path) in contact_walking_path.iter() {
            log::warn!(
                "Path with all team names: {:?}",
                path.iter()
                    .map(|contact| self.contact_list[*contact].team_name.clone())
                    .collect::<Vec<String>>()
            );
            for i in 0..path.len() - 1 {
                let contact_one = &self.contact_list[*path.get(i).unwrap()];
                let contact_two = &self.contact_list[*path.get(i + 1).unwrap()];

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

    pub fn top_plan(&self) -> Option<Plan> {
        let top_score = self.top_score.lock().unwrap().seed.clone();
        if (top_score.is_none()) {
            return None;
        }
        Some(self.seed_to_plan(top_score.unwrap()))
    }
}

impl<'calc> Course<'calc> {
    fn new(name: &'calc String, host_index: usize) -> Self {
        Course {
            name,
            host_index,
            guest_index_list: Vec::new(),
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

fn course_list_to_map(course_list: &Vec<Course>) -> HashMap<String, Vec<usize>> {
    let mut course_map = HashMap::new();
    for (index, course) in course_list.iter().enumerate() {
        course_map
            .entry(course.name.to_string())
            .or_insert_with(Vec::new)
            .push(index);
    }
    course_map
}

fn set_seen_people(
    seen_contact_map: &mut HashMap<usize, HashSet<usize>>,
    course: &Course,
    new_guest: usize,
) {
    {
        let seen_guest_set_guest = seen_contact_map.get_mut(&new_guest).unwrap();
        seen_guest_set_guest.insert(course.host_index);
    }
    {
        let seen_guest_set_host = seen_contact_map.get_mut(&course.host_index).unwrap();
        seen_guest_set_host.insert(new_guest);
    }

    for guest in &course.guest_index_list {
        let seen_guest_set = seen_contact_map.get_mut(&new_guest).unwrap();
        seen_guest_set.insert(*guest);

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
