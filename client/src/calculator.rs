use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::storage::{AddressData, ContactData, CookAndRunData, HostingData, PlanData};

#[derive(Debug, Clone)]
pub struct Calculator {
    contact_list: HashMap<Uuid, ContactData>,
    course_list: Vec<Uuid>,
    course_with_more_hosts: Option<Uuid>,
    start_point: Option<AddressData>,
    end_point: Option<AddressData>,
    top_plan: Arc<Mutex<Option<Plan>>>,
    should_stop: Arc<Mutex<bool>>,
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Plan {
    hosting_list: HashMap<Uuid /*Hosting ID */, HostingData>,
    walking_path: HashMap<Uuid /*Contact ID */, Vec<Uuid /*Hosting ID */>>,
    greatest_distance: f64,
}

#[derive(Debug)]
struct MeetingTracker {
    meetings: HashMap<Uuid, HashMap<Uuid, u32>>,
}

impl MeetingTracker {
    fn new() -> Self {
        Self {
            meetings: HashMap::new(),
        }
    }

    fn record_meeting(&mut self, person1: Uuid, person2: Uuid) {
        *self
            .meetings
            .entry(person1)
            .or_default()
            .entry(person2)
            .or_default() += 1;
        *self
            .meetings
            .entry(person2)
            .or_default()
            .entry(person1)
            .or_default() += 1;
    }

    fn get_meeting_count(&self, person1: Uuid, person2: Uuid) -> u32 {
        self.meetings
            .get(&person1)
            .and_then(|m| m.get(&person2))
            .copied()
            .unwrap_or(0)
    }

    fn has_met(&self, person1: Uuid, person2: Uuid) -> bool {
        self.get_meeting_count(person1, person2) > 0
    }

    fn total_meetings_for_person(&self, person: Uuid) -> u32 {
        self.meetings
            .get(&person)
            .map(|m| m.values().sum())
            .unwrap_or(0)
    }
}

impl Plan {
    fn to_plan_data(&self) -> PlanData {
        PlanData {
            id: Uuid::new_v4(),
            hosting_list: self
                .hosting_list
                .iter()
                .map(|(_, hosting)| hosting.clone())
                .collect(),
            walking_path: self.walking_path.clone(),
            greatest_distance: self.greatest_distance,
        }
    }

    fn new(
        start_point: &Option<AddressData>,
        end_point: &Option<AddressData>,
        course_sorted_list: &Vec<Uuid>,
        hosting_list: HashMap<Uuid, HostingData>,
        contact_list: &HashMap<Uuid, ContactData>,
    ) -> Self {
        let walking_path = Self::calculate_walking_path(course_sorted_list, &hosting_list);
        let greatest_distance = Self::calculate_fitness(
            start_point,
            end_point,
            &hosting_list,
            &walking_path,
            contact_list,
        );
        Plan {
            hosting_list,
            walking_path,
            greatest_distance,
        }
    }

    fn calculate_fitness(
        start_point: &Option<AddressData>,
        end_point: &Option<AddressData>,
        hosting_list: &HashMap<Uuid, HostingData>,
        walking_path: &HashMap<Uuid, Vec<Uuid>>,
        contact_list: &HashMap<Uuid, ContactData>,
    ) -> f64 {
        let mut fitness = 0.0;

        for (_, hosting_path) in walking_path.iter() {
            let mut current_fitness = 0.0;

            let mut hosting_iter = hosting_path.iter();
            let mut last_addr = Self::get_address(
                &contact_list,
                &hosting_list
                    .get(hosting_iter.next().expect("Expect first Hosting"))
                    .expect("Expect to find Hosting")
                    .host,
            );

            if let Some(start_point) = start_point.as_ref() {
                current_fitness = start_point.distance(last_addr);
            }

            loop {
                let next_hosting = hosting_iter.next();
                if let Some(next_hosting) = next_hosting {
                    let next_addr = Self::get_address(
                        &contact_list,
                        &hosting_list
                            .get(next_hosting)
                            .expect("Expect to find Hosting")
                            .host,
                    );

                    current_fitness = current_fitness + last_addr.distance(next_addr);

                    last_addr = next_addr;
                } else {
                    break;
                }
            }

            if let Some(end_point) = end_point.as_ref() {
                current_fitness = current_fitness + end_point.distance(last_addr);
            }

            if current_fitness > fitness {
                fitness = current_fitness;
            }
        }

        fitness
    }

    fn get_address<'a>(
        contact_list: &'a HashMap<Uuid, ContactData>,
        contact_id: &'a Uuid,
    ) -> &'a AddressData {
        &contact_list
            .get(contact_id)
            .expect("Expect to find Contact")
            .address
    }

    fn calculate_walking_path(
        course_sorted_list: &Vec<Uuid>,
        hosting_list: &HashMap<Uuid, HostingData>,
    ) -> HashMap<Uuid, Vec<Uuid>> {
        let mut walking_path: HashMap<
            Uuid, /*Contact ID */
            Vec<(Uuid /*Hosting ID */, Uuid /*Course ID */)>,
        > = HashMap::new();

        for hosting in hosting_list.values() {
            walking_path
                .entry(hosting.host)
                .or_default()
                .push((hosting.id, hosting.name));
            for &guest in &hosting.guest_list {
                walking_path
                    .entry(guest)
                    .or_default()
                    .push((hosting.id, hosting.name));
            }
        }

        // Sort the walking path by course order
        for (_, path) in walking_path.iter_mut() {
            path.sort_by_key(|&hosting| {
                course_sorted_list
                    .iter()
                    .position(|&course| course == hosting.1)
                    .expect("Expect to find Hosting ID in course list")
            });
        }

        walking_path
            .iter()
            .map(|(contact_id, path)| {
                (
                    *contact_id,
                    path.iter().map(|(hosting_id, _)| *hosting_id).collect(),
                )
            })
            .collect()
    }
}

impl Calculator {
    pub fn new(cook_and_run_data: &CookAndRunData) -> Result<Calculator, String> {
        println!("Creating Calculator");
        let mut course_list = cook_and_run_data.course_list.clone();
        course_list.sort_by(|a, b| a.time.cmp(&b.time));
        let course_list = course_list.iter().map(|c| c.id).collect();
        let calc = Calculator {
            contact_list: cook_and_run_data
                .contact_list
                .iter()
                .map(|c| (c.id, c.clone()))
                .collect(),
            course_list,
            course_with_more_hosts: cook_and_run_data.course_with_more_hosts.clone(),
            start_point: cook_and_run_data.start_point.clone().map(|s| s.address),
            end_point: cook_and_run_data.end_point.clone().map(|e| e.address),
            top_plan: Arc::new(Mutex::new(None)),
            should_stop: Arc::new(Mutex::new(false)),
        };
        let result = calc.check();
        if let Some(err) = result.err() {
            Err(err)
        } else {
            Ok(calc)
        }
    }

    pub fn calculate(&self) {
        println!("Starting deterministic calculation...");
        let contact_list = self.contact_list.clone();
        let course_list = self.course_list.clone();
        let start_point = self.start_point.clone();
        let end_point = self.end_point.clone();
        let course_with_more_hosts = self.course_with_more_hosts;

        let top_plan = Arc::clone(&self.top_plan);
        let should_stop = Arc::clone(&self.should_stop);

        // Reset should_stop
        *should_stop.lock().unwrap() = false;

        // thread::spawn(move || {
        // Deterministic calculation
        let result = Self::calculate_optimal_plan(
            &contact_list,
            &course_list,
            &start_point,
            &end_point,
            &course_with_more_hosts,
        );

        match result {
            Ok(hosting_map) => {
                println!("Found optimal PLAN with {} hostings", hosting_map.len());
                let plan = Plan::new(
                    &start_point,
                    &end_point,
                    &course_list,
                    hosting_map,
                    &contact_list,
                );
                println!("Plan fitness: {}", plan.greatest_distance);
                top_plan.lock().unwrap().replace(plan);
            }
            Err(err) => {
                println!("Error during optimal calculation: {}", err);
                // Fallback to relaxed constraints
                if let Ok(fallback_map) = Self::calculate_with_relaxed_constraints(
                    &contact_list,
                    &course_list,
                    &start_point,
                    &end_point,
                    &course_with_more_hosts,
                ) {
                    println!("Found fallback PLAN with {} hostings", fallback_map.len());
                    let plan = Plan::new(
                        &start_point,
                        &end_point,
                        &course_list,
                        fallback_map,
                        &contact_list,
                    );
                    println!("Fallback plan fitness: {}", plan.greatest_distance);
                    top_plan.lock().unwrap().replace(plan);
                } else {
                    println!("Even fallback calculation failed");
                }
            }
        }
        // });
    }

    fn calculate_optimal_plan(
        contact_list: &HashMap<Uuid, ContactData>,
        course_list: &Vec<Uuid>,
        start_point: &Option<AddressData>,
        end_point: &Option<AddressData>,
        course_with_more_hosts: &Option<Uuid>,
    ) -> Result<HashMap<Uuid, HostingData>, String> {
        // Phase 1: Optimal host assignment
        let host_assignments = Self::assign_hosts_optimally(
            contact_list,
            course_list,
            start_point,
            end_point,
            course_with_more_hosts,
        );

        // Phase 2: Deterministic guest assignment
        Self::assign_guests_deterministically(host_assignments, contact_list, course_list)
    }

    fn calculate_with_relaxed_constraints(
        contact_list: &HashMap<Uuid, ContactData>,
        course_list: &Vec<Uuid>,
        start_point: &Option<AddressData>,
        end_point: &Option<AddressData>,
        course_with_more_hosts: &Option<Uuid>,
    ) -> Result<HashMap<Uuid, HostingData>, String> {
        // Same as optimal but with relaxed meeting constraints
        let host_assignments = Self::assign_hosts_optimally(
            contact_list,
            course_list,
            start_point,
            end_point,
            course_with_more_hosts,
        );

        Self::assign_guests_with_relaxed_constraints(host_assignments, contact_list, course_list)
    }

    fn assign_hosts_optimally(
        contact_list: &HashMap<Uuid, ContactData>,
        course_list: &Vec<Uuid>,
        start_point: &Option<AddressData>,
        end_point: &Option<AddressData>,
        course_with_more_hosts: &Option<Uuid>,
    ) -> HashMap<Uuid, HostingData> {
        println!(
            "Assigning {} hosting with distance calculation",
            contact_list.len()
        );
        let mut contact_start_distance = Vec::new();
        let mut contact_goal_distance = Vec::new();

        for contact in contact_list.values() {
            if let Some(start_point) = start_point.as_ref() {
                let start_distance = start_point.distance(&contact.address);
                if let Some(end_point) = end_point.as_ref() {
                    let end_distance = end_point.distance(&contact.address);
                    if start_distance < end_distance {
                        contact_start_distance.push((contact, start_distance));
                    } else {
                        contact_goal_distance.push((contact, end_distance));
                    }
                } else {
                    contact_start_distance.push((contact, start_distance));
                }
            } else if let Some(end_point) = end_point.as_ref() {
                let end_distance = end_point.distance(&contact.address);
                contact_goal_distance.push((contact, end_distance));
            }
        }

        contact_start_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        contact_goal_distance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let contact_list = [contact_start_distance, contact_goal_distance].concat();
        println!("List of hosts sorted by distance: {:?}", contact_list);

        let hosts_per_course = contact_list.len() / course_list.len();
        let overhang = contact_list.len() % course_list.len();
        let mut hosting_list = HashMap::new();

        let mut current_course_index = 0;
        let mut current_course_hosts = 0;

        println!("Assigne {} hosts per course", hosts_per_course);
        println!("Overhang: {}", overhang);

        for (contact, _) in contact_list {
            let course_id = course_list[current_course_index];
            let hosting_data = HostingData {
                id: Uuid::new_v4(),
                host: contact.id,
                name: course_id,
                guest_list: Vec::new(),
            };
            hosting_list.insert(hosting_data.id, hosting_data);

            current_course_hosts += 1;
            if current_course_hosts >= hosts_per_course {
                if course_with_more_hosts.is_some()
                    && course_with_more_hosts.expect("Expect uuid!") == course_id
                {
                    if current_course_hosts == hosts_per_course + overhang {
                        current_course_index += 1;
                        current_course_hosts = 0;
                    }
                } else {
                    current_course_index += 1;
                    current_course_hosts = 0;
                }
            }
        }

        println!("Hosting list created with {} entries", hosting_list.len());
        println!(
            "Hosting list: {:?}",
            hosting_list.values().map(|h| h.id).collect::<Vec<Uuid>>()
        );

        hosting_list
    }

    fn assign_guests_deterministically(
        host_assignments: HashMap<Uuid, HostingData>,
        contact_list: &HashMap<Uuid, ContactData>,
        course_list: &Vec<Uuid>,
    ) -> Result<HashMap<Uuid, HostingData>, String> {
        let mut new_hosting_map = HashMap::new();
        let mut meeting_tracker = MeetingTracker::new();

        // Group hostings by course
        let mut course_hosting_map: HashMap<Uuid, Vec<&HostingData>> = HashMap::new();
        for hosting in host_assignments.values() {
            course_hosting_map
                .entry(hosting.name)
                .or_default()
                .push(hosting);
        }

        // Process each course
        for &course_id in course_list {
            if let Some(hosting_list) = course_hosting_map.get(&course_id) {
                let host_ids: HashSet<Uuid> = hosting_list.iter().map(|h| h.host).collect();
                let mut available_guests: Vec<Uuid> = contact_list
                    .keys()
                    .filter(|&id| !host_ids.contains(id))
                    .copied()
                    .collect();

                // Sort guests by total meetings (least meetings first)
                available_guests
                    .sort_by_key(|&guest_id| meeting_tracker.total_meetings_for_person(guest_id));

                let guests_per_host = available_guests.len() / hosting_list.len();
                let extra_guests = available_guests.len() % hosting_list.len();
                let mut guest_index = 0;

                for (host_index, hosting) in hosting_list.iter().enumerate() {
                    let mut guest_list = Vec::new();
                    let mut guests_for_this_host = guests_per_host;
                    if host_index < extra_guests {
                        guests_for_this_host += 1;
                    }

                    for _ in 0..guests_for_this_host {
                        if guest_index >= available_guests.len() {
                            return Err("Not enough guests available".to_string());
                        }

                        let best_guest = Self::find_best_guest(
                            hosting.host,
                            &available_guests[guest_index..],
                            &meeting_tracker,
                        )
                        .ok_or("No suitable guest found")?;

                        guest_list.push(best_guest);

                        // Remove guest from available list
                        if let Some(pos) = available_guests.iter().position(|&x| x == best_guest) {
                            available_guests.remove(pos);
                            if pos < guest_index {
                                guest_index = guest_index.saturating_sub(1);
                            }
                        }

                        // Record meetings
                        meeting_tracker.record_meeting(hosting.host, best_guest);
                        for &other_guest in &guest_list {
                            if other_guest != best_guest {
                                meeting_tracker.record_meeting(best_guest, other_guest);
                            }
                        }
                    }

                    let mut hosting_data = (*hosting).clone();
                    hosting_data.guest_list = guest_list;
                    new_hosting_map.insert(hosting_data.id, hosting_data);
                }
            }
        }

        Ok(new_hosting_map)
    }

    fn assign_guests_with_relaxed_constraints(
        host_assignments: HashMap<Uuid, HostingData>,
        contact_list: &HashMap<Uuid, ContactData>,
        course_list: &Vec<Uuid>,
    ) -> Result<HashMap<Uuid, HostingData>, String> {
        let mut new_hosting_map = HashMap::new();

        // Group hostings by course
        let mut course_hosting_map: HashMap<Uuid, Vec<&HostingData>> = HashMap::new();
        for hosting in host_assignments.values() {
            course_hosting_map
                .entry(hosting.name)
                .or_default()
                .push(hosting);
        }

        // Simple round-robin assignment without meeting constraints
        for &course_id in course_list {
            if let Some(hosting_list) = course_hosting_map.get(&course_id) {
                let host_ids: HashSet<Uuid> = hosting_list.iter().map(|h| h.host).collect();
                let available_guests: Vec<Uuid> = contact_list
                    .keys()
                    .filter(|&id| !host_ids.contains(id))
                    .copied()
                    .collect();

                let guests_per_host = available_guests.len() / hosting_list.len();
                let extra_guests = available_guests.len() % hosting_list.len();
                let mut guest_index = 0;

                for (host_index, hosting) in hosting_list.iter().enumerate() {
                    let mut guest_list = Vec::new();
                    let mut guests_for_this_host = guests_per_host;
                    if host_index < extra_guests {
                        guests_for_this_host += 1;
                    }

                    for _ in 0..guests_for_this_host {
                        if guest_index < available_guests.len() {
                            guest_list.push(available_guests[guest_index]);
                            guest_index += 1;
                        }
                    }

                    let mut hosting_data = (*hosting).clone();
                    hosting_data.guest_list = guest_list;
                    new_hosting_map.insert(hosting_data.id, hosting_data);
                }
            }
        }

        Ok(new_hosting_map)
    }

    fn find_best_guest(
        host: Uuid,
        available_guests: &[Uuid],
        meeting_tracker: &MeetingTracker,
    ) -> Option<Uuid> {
        // Priority 1: Guests who have never met the host
        for &guest in available_guests {
            if !meeting_tracker.has_met(host, guest) {
                return Some(guest);
            }
        }

        // Priority 2: Guests who have met the host only once
        for &guest in available_guests {
            if meeting_tracker.get_meeting_count(host, guest) == 1 {
                return Some(guest);
            }
        }

        // Fallback: Any available guest
        available_guests.first().copied()
    }

    pub fn stop(&self) {
        *self.should_stop.lock().unwrap() = true;
    }

    pub fn get_top_plan(&self) -> Option<PlanData> {
        let plan = self.top_plan.lock().unwrap().clone();
        plan.map(|p| p.to_plan_data())
    }
}

impl Calculator {
    fn check(&self) -> Result<(), String> {
        self.check_min_number_of_contacts()?;
        self.check_overhang()?;
        Ok(())
    }

    fn check_min_number_of_contacts(&self) -> Result<(), String> {
        if self.contact_list.len() < self.course_list.len() {
            Err("There can't be more courses than contact's!".to_string())
        } else {
            Ok(())
        }
    }

    fn check_overhang(&self) -> Result<(), String> {
        if self.contact_list.len() % self.course_list.len() != 0
            && self.course_with_more_hosts.is_none()
        {
            Err("A course with more hosts has to be set!".to_string())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::{
        calculator::{Calculator, MeetingTracker},
        storage::{ContactData, CookAndRunData, CourseData},
    };

    #[test]
    fn test_sorting_in_constructor() {
        let course_1 = CourseData {
            id: Uuid::new_v4(),
            name: "Course 1".to_string(),
            time: chrono::NaiveTime::from_hms_opt(5, 0, 0).unwrap(),
        };
        let course_2 = CourseData {
            id: Uuid::new_v4(),
            name: "Course 2".to_string(),
            time: chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        };
        let course_3 = CourseData {
            id: Uuid::new_v4(),
            name: "Course 3".to_string(),
            time: chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
        };

        let mut cook_and_run_data = CookAndRunData::default();
        cook_and_run_data.course_list.push(course_2.clone());
        cook_and_run_data.course_list.push(course_1.clone());
        cook_and_run_data.course_list.push(course_3.clone());

        cook_and_run_data.contact_list.push(ContactData::default());
        cook_and_run_data.contact_list.push(ContactData::default());
        cook_and_run_data.contact_list.push(ContactData::default());

        let calculator = Calculator::new(&cook_and_run_data);
        assert!(
            calculator.is_ok(),
            "Calculator creation failed: {:?}",
            calculator.err()
        );

        let calculator = calculator.unwrap();
        assert_eq!(
            calculator.course_list.len(),
            3,
            "Expected 3 courses, found {}",
            calculator.course_list.len()
        );

        assert_eq!(
            calculator.course_list[0], course_1.id,
            "First course should be 'Course 1'"
        );
        assert_eq!(
            calculator.course_list[1], course_2.id,
            "Second course should be 'Course 2'"
        );
        assert_eq!(
            calculator.course_list[2], course_3.id,
            "Third course should be 'Course 3'"
        );
    }

    #[test]
    fn test_meeting_tracker() {
        let mut tracker = MeetingTracker::new();
        let person1 = Uuid::new_v4();
        let person2 = Uuid::new_v4();

        assert!(!tracker.has_met(person1, person2));
        tracker.record_meeting(person1, person2);
        assert!(tracker.has_met(person1, person2));
        assert_eq!(tracker.get_meeting_count(person1, person2), 1);
    }
}
