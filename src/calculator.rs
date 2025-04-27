use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::{self},
    time::Instant,
};

use log::{debug, info};

use rand::Rng;

use colored::Colorize;

use crate::contact::Contact;

#[derive(Debug)]
pub struct Calculator {
    pub top_plan: Arc<Mutex<Option<Plan>>>,
    pub start_time: Option<Instant>,
    pub iterations: Arc<AtomicUsize>,
    config: CalculatorConfig,
    calculating: Arc<Mutex<bool>>,
}

#[derive(Debug)]
pub struct CalculatorConfig {
    start_point: Option<(i32, i32)>,
    goal_point: Option<(i32, i32)>,
    course_name_list: Vec<String>,
    course_with_more_hosts: Option<String>,
    contact_list: Vec<Contact>,
}

#[derive(Debug)]
struct CalculatorConfigInternal {
    start_point: Option<(i32, i32)>,
    goal_point: Option<(i32, i32)>,
    course_name_list: Vec<Rc<String>>,
    course_with_more_hosts: Option<String>,
    contact_list: Vec<Rc<Contact>>,
}

struct PlanInternal {
    seed: Vec<u8>,
    course_map: HashMap<Rc<String>, Vec<Rc<CourseInternal>>>,
    walking_path: HashMap<Rc<Contact>, HashSet<Rc<CourseInternal>>>,
    score: f64,
}

impl PlanInternal {
    fn to_plan(&self) -> Plan {
        Plan {
            seed: self.seed.clone(),
            course_map: self
                .course_map
                .iter()
                .map(|(name, course_list)| {
                    (
                        name.as_ref().clone(),
                        course_list
                            .iter()
                            .map(Rc::as_ref)
                            .map(CourseInternal::to_course)
                            .collect(),
                    )
                })
                .collect(),
            walking_path: self
                .walking_path
                .iter()
                .map(|(contact, course_list)| {
                    (
                        contact.as_ref().clone(),
                        course_list
                            .iter()
                            .map(Rc::as_ref)
                            .map(CourseInternal::to_course)
                            .collect(),
                    )
                })
                .collect(),
            score: self.score,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub seed: Vec<u8>,
    pub course_map: HashMap<String, Vec<Course>>,
    pub walking_path: HashMap<Contact, Vec<Course>>,
    pub score: f64,
}

#[derive(Hash, Debug, Clone)]
pub struct Course {
    pub name: String,
    pub host: Contact,
    pub guest_list: Vec<Contact>,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct CourseInternal {
    pub name: Rc<String>,
    pub host: Rc<Contact>,
    pub guest_list: Vec<Rc<Contact>>,
}

impl CourseInternal {
    fn to_course(&self) -> Course {
        Course {
            name: self.name.as_ref().clone(),
            host: self.host.as_ref().clone(),
            guest_list: self
                .guest_list
                .iter()
                .map(|guest| guest.as_ref().clone())
                .collect(),
        }
    }
}

impl CalculatorConfig {
    pub fn new_with_start_and_goal(
        start_point: Option<(i32, i32)>,
        goal_point: Option<(i32, i32)>,
        course_with_more_hosts: Option<String>,
        course_name_list: Vec<String>,
        contact_list: Vec<Contact>,
    ) -> Self {
        CalculatorConfig {
            start_point,
            goal_point,
            course_with_more_hosts,
            course_name_list,
            contact_list: contact_list,
        }
    }
    pub fn new(
        course_name_list: Vec<String>,
        contact_list: Vec<Contact>,
        course_with_more_hosts: Option<String>,
    ) -> Self {
        CalculatorConfig {
            start_point: None,
            goal_point: None,
            course_with_more_hosts,
            course_name_list,
            contact_list,
        }
    }

    fn get_internal(&self) -> CalculatorConfigInternal {
        CalculatorConfigInternal {
            start_point: self.start_point,
            goal_point: self.goal_point,
            course_name_list: self
                .course_name_list
                .iter()
                .map(|name| Rc::new(name.clone()))
                .collect(),
            course_with_more_hosts: self.course_with_more_hosts.clone(),
            contact_list: self
                .contact_list
                .iter()
                .map(|contact| Rc::new(contact.clone()))
                .collect(),
        }
    }

    fn clone(&self) -> CalculatorConfig {
        CalculatorConfig {
            start_point: self.start_point,
            goal_point: self.goal_point,
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
            top_plan: Arc::new(Mutex::new(None)),
            calculating: Arc::new(Mutex::new(true)),
            start_time: None,
            iterations: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn calculate(&mut self) {
        log::info!("Start calculation");
        self.iterations.store(0, Ordering::SeqCst);
        self.start_time = Some(Instant::now());
        let number_of_threads = 5;
        *self
            .calculating
            .lock()
            .expect("Expect calculating to be set!") = true;
        for index in 0..number_of_threads {
            let config = self.config.clone();
            let top_plan = Arc::clone(&self.top_plan);
            let calculating = Arc::clone(&self.calculating);
            let iteration = Arc::clone(&self.iterations);
            thread::spawn(move || {
                info!(
                    "Start calculation of thread {}/{}",
                    index + 1,
                    number_of_threads
                );
                calcutate_job(config.get_internal(), calculating, top_plan, iteration);
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
}

fn calcutate_job(
    config: CalculatorConfigInternal,
    calculating: Arc<Mutex<bool>>,
    top_plan: Arc<Mutex<Option<Plan>>>,
    iteration: Arc<AtomicUsize>,
) {
    let number_of_seeds = 1_000;

    let mut list_of_seeds = Vec::new();
    for _ in 0..number_of_seeds {
        list_of_seeds.push(generate_seed());
    }

    loop {
        iteration.fetch_add(1, Ordering::SeqCst);
        let mut list_of_plans: Vec<PlanInternal> = list_of_seeds
            .iter()
            .map(|seed| seed_to_plan(&config, seed.clone()))
            .collect();
        list_of_plans.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        if list_of_plans[0].score != f64::MAX {
            let mut current_top_score = top_plan.lock().unwrap();

            match current_top_score.as_ref() {
                None => {
                    // No score yet → set directly
                    log::info!(
                        "Found first best plan with score: {}",
                        list_of_plans[0].score
                    );
                    *current_top_score = Some(list_of_plans[0].to_plan());
                }
                Some(existing_score) if list_of_plans[0].score < existing_score.score => {
                    // New score is better → overwrite
                    log::info!("Found new best plan with score: {}", list_of_plans[0].score);
                    *current_top_score = Some(list_of_plans[0].to_plan());
                }
                _ => {
                    // Score is not better → do nothing
                }
            }
        }
        if !*calculating.lock().expect("Expact to find calculating flag") {
            break;
        }
        list_of_seeds = generate_seed_from_plan_list(list_of_plans);
    }
}

fn seed_to_plan(config: &CalculatorConfigInternal, seed: Vec<u8>) -> PlanInternal {
    debug!("Convert seed to Plan");
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
    config: &CalculatorConfigInternal,
    seed: &Vec<u8>,
) -> Option<HashMap<Rc<String>, Vec<Rc<CourseInternal>>>> {
    let mut course_map = HashMap::new();
    let mut seen_contact_map = HashMap::new();
    let mut seen_contact_map_second_time = HashMap::new();

    for contact in config.contact_list.iter() {
        seen_contact_map.insert(Rc::clone(contact), HashSet::new());
        seen_contact_map_second_time.insert(Rc::clone(contact), HashSet::new());
    }

    let mut seed_index = 0;

    let mut possible_host_list = config
        .contact_list
        .iter()
        .cloned()
        .collect::<Vec<Rc<Contact>>>();
    for course_name in config.course_name_list.iter() {
        debug!("Calculating course \"{}\"", course_name);
        debug!(
            "List of people seen by each person:\n{:?}",
            seen_contact_map
                .iter()
                .map(|(k, v)| (
                    Rc::clone(k),
                    v.iter()
                        .map(|contact: &Rc<Contact>| Rc::clone(contact))
                        .collect::<Vec<Rc<Contact>>>()
                ))
                .collect::<HashMap<Rc<Contact>, Vec<Rc<Contact>>>>()
        );

        debug!(
            "List of second time people seen by each person:\n{:?}",
            seen_contact_map_second_time
                .iter()
                .map(|(k, v)| (
                    k.team_name.as_str(),
                    v.iter()
                        .map(|x: &Rc<Contact>| x.team_name.as_str())
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
        let mut possible_guest_list = config
            .contact_list
            .iter()
            .cloned()
            .collect::<Vec<Rc<Contact>>>();
        let mut contact_in_course: HashSet<Rc<Contact>> = HashSet::new();

        let number_of_courses = config.contact_list.len() / config.course_name_list.len()
            + if config.course_with_more_hosts.as_deref() == Some(course_name)
                && config.contact_list.len() % config.course_name_list.len() != 0
            {
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
            let host = Rc::clone(
                possible_host_in_course_list
                    .get(host_index)
                    .expect("Expected host to find in possible host list for this course!"),
            );
            //Remove host from possible host list in course
            possible_host_in_course_list.remove(host_index);

            //Remove host from possible host list
            let remove_host_index = possible_host_list
                .iter()
                .position(|x| x.eq(&host))
                .expect("Expected host in list of possible hosts!");
            possible_host_list.remove(remove_host_index);

            //Remove host from possible guest list
            let remove_guest_index = possible_guest_list
                .iter()
                .position(|x| x.eq(&host))
                .expect("Expected host in list of possible guests!");
            possible_guest_list.remove(remove_guest_index);

            seed_index += 1;

            set_seen_people(
                &mut seen_contact_map,
                &mut contact_in_course,
                &guest_list,
                Rc::clone(&host),
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
                    Rc::clone(&host),
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
                name: Rc::clone(course_name),
                host: Rc::clone(&host),
                guest_list: guest_list,
            };

            course_map
                .entry(Rc::clone(course_name))
                .or_insert_with(Vec::new)
                .push(Rc::new(course));
        }
    }
    Some(course_map)
}

fn calc_score(
    config: &CalculatorConfigInternal,
    contact_walking_path_set: &HashMap<Rc<Contact>, HashSet<Rc<CourseInternal>>>,
) -> f64 {
    let mut longest_distance = 0_f64;

    for (_, path) in contact_walking_path_set.iter() {
        let mut path_iter = path.iter();
        let mut distance = 0_f64;
        let mut contact_from;
        let mut contact_to;

        let first_course = path_iter.next().expect("Expected first course in path!");
        contact_to = &first_course.host;
        if config.start_point.is_some() {
            distance += calc_distance(
                config
                    .start_point
                    .expect("Expected start point latitude!")
                    .0,
                config
                    .start_point
                    .expect("Expected start point longitude!")
                    .1,
                contact_to.latitude,
                contact_to.longitude,
            );
        }
        while let Some(course) = path_iter.next() {
            contact_from = contact_to;
            contact_to = &course.host;
            distance += calc_distance(
                contact_from.latitude,
                contact_from.longitude,
                contact_to.latitude,
                contact_to.longitude,
            );
        }

        if config.goal_point.is_some() {
            distance += calc_distance(
                contact_to.latitude,
                contact_to.longitude,
                config.goal_point.expect("Expected goal point latitude!").0,
                config.goal_point.expect("Expected goal point longitude!").1,
            );
        }
        if distance > longest_distance {
            longest_distance = distance;
        }
    }
    longest_distance
}

fn calc_walking_path(
    course_map: &HashMap<Rc<String>, Vec<Rc<CourseInternal>>>,
) -> HashMap<Rc<Contact>, HashSet<Rc<CourseInternal>>> {
    let contact_map = course_map_to_contact_map(course_map);
    let mut contact_walking_path = HashMap::new();

    for (_, course_list) in contact_map.iter() {
        for course in course_list {
            let path = contact_walking_path
                .entry(Rc::clone(&course.host))
                .or_insert(HashSet::new());
            path.insert(Rc::clone(course));
            for guest in course.guest_list.iter() {
                let path = contact_walking_path
                    .entry(Rc::clone(guest))
                    .or_insert(HashSet::new());
                path.insert(Rc::clone(course));
            }
        }
    }

    contact_walking_path
}

fn get_contact(
    possible_guest_list: &Vec<Rc<Contact>>,
    seed_id: u8,
    contact_in_course: &HashSet<Rc<Contact>>,
    seen_contact_map: &HashMap<Rc<Contact>, HashSet<Rc<Contact>>>,
) -> Option<Rc<Contact>> {
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
        return Some(Rc::clone(contact));
    }
    None
}

fn calc_distance(
    start_point_latitude: i32,
    start_point_longitude: i32,
    goal_point_latitude: i32,
    goal_point_longitude: i32,
) -> f64 {
    f64::sqrt(
        (((goal_point_latitude as i64 - start_point_latitude as i64).pow(2)
            + (goal_point_longitude as i64 - start_point_longitude as i64).pow(2)) as f64)
            .sqrt(),
    )
}

fn course_map_to_contact_map(
    course_map: &HashMap<Rc<String>, Vec<Rc<CourseInternal>>>,
) -> HashMap<Rc<Contact>, Vec<Rc<CourseInternal>>> {
    let mut contact_map = HashMap::new();
    for course_list in course_map.values() {
        for course in course_list.iter() {
            contact_map
                .entry(Rc::clone(&course.host))
                .or_insert_with(Vec::new)
                .push(Rc::clone(course));
            for guest in course.guest_list.iter() {
                contact_map
                    .entry(Rc::clone(guest))
                    .or_insert_with(Vec::new)
                    .push(Rc::clone(course));
            }
        }
    }
    contact_map
}

fn set_seen_people(
    seen_contact_map: &mut HashMap<Rc<Contact>, HashSet<Rc<Contact>>>,
    contact_in_course: &mut HashSet<Rc<Contact>>,
    guest_list: &Vec<Rc<Contact>>,
    new_contact: Rc<Contact>,
) {
    contact_in_course.insert(Rc::clone(&new_contact));
    guest_list.iter().for_each(|guest| {
        seen_contact_map
            .get_mut(&new_contact)
            .expect("Expected to find seen contact of new contact!")
            .insert(Rc::clone(guest));
        seen_contact_map
            .get_mut(guest)
            .expect("Expected to find seen contact of guest!")
            .insert(Rc::clone(&new_contact));
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
