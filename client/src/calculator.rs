use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::storage::{AddressData, ContactData, CookAndRunData, CourseData, HostingData, PlanData};

// Interne Datenstrukturen für den Algorithmus
#[derive(Debug)]
pub struct CookAndRunPlan {
    pub hosting_list: Vec<HostingData>,
    pub total_distance: f64,
    pub meeting_violations: usize,
    pub walking_paths: HashMap<Uuid, Vec<Uuid>>,
}

#[derive(Clone)]
struct PlanningState {
    hosting_list: Vec<HostingData>,
    host_assignments: HashMap<Uuid, bool>, // Contact ID -> has_hosted
    participant_meetings: HashMap<Uuid, HashSet<Uuid>>, // Contact ID -> Set of met Contact IDs
    course_idx: usize,
}
pub struct Calculator {
    cook_and_run_data: CookAndRunData,
    calculating: Arc<Mutex<bool>>,
    top_plan: Arc<Mutex<Option<PlanData>>>,
}

fn check_data(cook_and_run_data: &CookAndRunData) -> Result<(), String> {
    check_contact_list(&cook_and_run_data.contact_list)?;
    check_course_list(&cook_and_run_data.course_list)?;
    check_ratio(
        cook_and_run_data.contact_list.len(),
        cook_and_run_data.course_list.len(),
    )?;
    if let Some(start_point) = &cook_and_run_data.start_point {
        check_address(&start_point.address)?;
    }
    if let Some(end_point) = &cook_and_run_data.end_point {
        check_address(&end_point.address)?;
    }

    Ok(())
}

fn check_ratio(number_of_contacts: usize, number_of_courses: usize) -> Result<(), String> {
    if number_of_courses <= number_of_contacts {
        Ok(())
    } else {
        Err("There can't be more courses than contacts".to_string())
    }
}

fn check_course_list(course_list: &Vec<CourseData>) -> Result<(), String> {
    for course_data in course_list {
        let result = check_course(course_data);
        if result.is_err() {
            return result;
        }
    }
    Ok(())
}

fn check_course(course_data: &CourseData) -> Result<(), String> {
    if course_data.id.is_nil() {
        return Err("Course-Id have to be set!".to_string());
    }
    Ok(())
}

fn check_contact_list(contact_list: &Vec<ContactData>) -> Result<(), String> {
    for contact_data in contact_list {
        let result = check_contact(contact_data);
        if result.is_err() {
            return result;
        }
    }
    Ok(())
}

fn check_contact(contact_data: &ContactData) -> Result<(), String> {
    if contact_data.id.is_nil() {
        return Err("Contact-Id have to be set!".to_string());
    }
    check_address(&contact_data.address)?;
    Ok(())
}

fn check_address(address: &AddressData) -> Result<(), String> {
    if address.latitude.eq(&0_f64) {
        return Err("Latitude have to be set!".to_string());
    }
    if address.longitude.eq(&0_f64) {
        return Err("Longitude have to be set!".to_string());
    }
    Ok(())
}

pub struct CookAndRunSolver {
    contacts: Vec<ContactData>,
    courses: Vec<CourseData>,
    max_group_size: usize,
    min_group_size: usize,
    course_with_more_hosts: Option<Uuid>,
}

impl CookAndRunSolver {
    pub fn new(
        contacts: Vec<ContactData>,
        courses: Vec<CourseData>,
        course_with_more_hosts: Option<Uuid>,
    ) -> Self {
        let num_participants = contacts.len();

        // Optimierte Gruppengrößenberechnung für größere Gruppen
        let (min_size, max_size) = if num_participants <= 12 {
            (3, 4)
        } else if num_participants <= 24 {
            (4, 5)
        } else if num_participants <= 40 {
            (5, 6)
        } else {
            (6, 8)
        };

        Self {
            contacts,
            courses,
            max_group_size: max_size,
            min_group_size: min_size,
            course_with_more_hosts,
        }
    }

    pub fn solve(&self) -> Result<PlanData, String> {
        if self.contacts.len() < 3 {
            return Err("Mindestens 3 Teilnehmer erforderlich".to_string());
        }

        if self.courses.len() < 2 {
            return Err("Mindestens 2 Kurse erforderlich".to_string());
        }

        // Schnelle Machbarkeitsprüfung
        if !self.is_feasible() {
            return Err("Problem ist nicht lösbar mit den gegebenen Parametern".to_string());
        }

        println!(
            "Starte Optimierung für {} Teilnehmer und {} Kurse...",
            self.contacts.len(),
            self.courses.len()
        );

        // Verwende Greedy-Algorithmus als Basis
        let greedy_plan = self.greedy_solve()?;
        println!(
            "Greedy-Lösung gefunden. Violations: {}",
            greedy_plan.meeting_violations
        );

        // Verbessere mit lokaler Suche falls nötig
        let improved_plan = if greedy_plan.meeting_violations > 0 {
            println!("Verbessere Lösung mit lokaler Suche...");
            self.local_search_improve(greedy_plan)?
        } else {
            greedy_plan
        };

        println!(
            "Finale Lösung: {} Violations, Distanz: {:.2}",
            improved_plan.meeting_violations, improved_plan.total_distance
        );

        // Konvertiere zu PlanData
        let plan_data = PlanData {
            id: Uuid::new_v4(),
            hosting_list: improved_plan.hosting_list,
            walking_path: improved_plan.walking_paths,
            greatest_distance: improved_plan.total_distance,
        };

        Ok(plan_data)
    }

    pub fn get_plan(&self) -> Option<PlanData> {
        let plan = self.solve();
        if plan.is_ok() {
            return Some(plan.expect("Expect plan"));
        }
        None
    }

    fn is_feasible(&self) -> bool {
        let n = self.contacts.len();
        let c = self.courses.len();
        let max_groups_per_course = (n + self.min_group_size - 1) / self.min_group_size;

        // Prüfe ob genug Hosts verfügbar sind
        // Berücksichtige dass ein Kurs mehr Hosts haben kann
        let total_host_slots = if self.course_with_more_hosts.is_some() {
            max_groups_per_course * c + max_groups_per_course
        } else {
            max_groups_per_course * c
        };

        n >= total_host_slots
    }

    fn greedy_solve(&self) -> Result<CookAndRunPlan, String> {
        let mut state = PlanningState {
            hosting_list: Vec::new(),
            host_assignments: HashMap::new(),
            participant_meetings: HashMap::new(),
            course_idx: 0,
        };

        // Initialisiere Host-Assignments und Meeting-Tracking
        for contact in &self.contacts {
            state.host_assignments.insert(contact.id, false);
            state
                .participant_meetings
                .insert(contact.id, HashSet::new());
        }

        // Verarbeite jeden Kurs
        for course in &self.courses {
            self.assign_course_greedy(course, &mut state)?;
            state.course_idx += 1;
        }

        let total_distance = self.calculate_total_distance(&state.hosting_list);
        let meeting_violations = self.count_meeting_violations(&state.hosting_list);
        let walking_paths = self.generate_walking_paths(&state.hosting_list);

        Ok(CookAndRunPlan {
            hosting_list: state.hosting_list,
            total_distance,
            meeting_violations,
            walking_paths,
        })
    }

    fn assign_course_greedy(
        &self,
        course: &CourseData,
        state: &mut PlanningState,
    ) -> Result<(), String> {
        let mut optimal_group_size = self.calculate_optimal_group_size();

        // Erhöhe Anzahl der Gruppen falls dies der Kurs mit mehr Hosts ist
        let is_extra_host_course = self.course_with_more_hosts.as_ref() == Some(&course.id);
        let groups_needed = if is_extra_host_course {
            let base_groups = (self.contacts.len() + optimal_group_size - 1) / optimal_group_size;
            // Verdoppele die Anzahl der Gruppen für diesen Kurs
            base_groups * 2
        } else {
            (self.contacts.len() + optimal_group_size - 1) / optimal_group_size
        };

        // Passe Gruppengröße an wenn wir mehr Gruppen haben
        if is_extra_host_course {
            optimal_group_size = optimal_group_size.max(2).min(self.max_group_size / 2);
        }

        // Wähle Hosts strategisch
        let hosts = self.select_hosts_strategically(groups_needed, &state.host_assignments)?;

        // Markiere Hosts als verwendet
        for &host_id in &hosts {
            state.host_assignments.insert(host_id, true);
        }

        // Erstelle Gruppen mit Greedy-Zuordnung
        let mut remaining_participants: Vec<Uuid> = self
            .contacts
            .iter()
            .filter(|c| !hosts.contains(&c.id))
            .map(|c| c.id)
            .collect();

        for &host_id in &hosts {
            let mut group = vec![host_id];

            // Fülle Gruppe optimal auf
            while group.len() < optimal_group_size && !remaining_participants.is_empty() {
                let best_guest = self.find_best_guest_greedy(
                    host_id,
                    &remaining_participants,
                    &state.participant_meetings,
                    &group,
                );

                if let Some(guest_id) = best_guest {
                    group.push(guest_id);
                    remaining_participants.retain(|&id| id != guest_id);
                } else {
                    break;
                }
            }

            // Aktualisiere Meetings
            self.update_meetings(&group, state);

            let guests = group.into_iter().skip(1).collect();
            state.hosting_list.push(HostingData {
                id: Uuid::new_v4(),
                name: course.id,
                host: host_id,
                guest_list: guests,
            });
        }

        // Verteile übrige Teilnehmer ausgewogen
        self.distribute_remaining_participants(&remaining_participants, state, &course.id);

        Ok(())
    }

    fn calculate_optimal_group_size(&self) -> usize {
        let n = self.contacts.len();
        let target_size = (n as f64 / self.estimate_groups_needed() as f64).round() as usize;
        target_size.clamp(self.min_group_size, self.max_group_size)
    }

    fn estimate_groups_needed(&self) -> usize {
        let n = self.contacts.len();
        (n + self.max_group_size - 1) / self.max_group_size
    }

    fn select_hosts_strategically(
        &self,
        groups_needed: usize,
        host_assignments: &HashMap<Uuid, bool>,
    ) -> Result<Vec<Uuid>, String> {
        let mut available_hosts: Vec<Uuid> = self
            .contacts
            .iter()
            .filter(|c| !host_assignments.get(&c.id).unwrap_or(&false))
            .map(|c| c.id)
            .collect();

        if available_hosts.len() < groups_needed {
            return Err("Nicht genug verfügbare Hosts".to_string());
        }

        // Sortiere Hosts nach geographischer Verteilung für bessere Distanzoptimierung
        available_hosts.sort_by(|&a, &b| {
            let contact_a = self.contacts.iter().find(|c| c.id == a).unwrap();
            let contact_b = self.contacts.iter().find(|c| c.id == b).unwrap();

            let coord_a = (contact_a.address.latitude, contact_a.address.longitude);
            let coord_b = (contact_b.address.latitude, contact_b.address.longitude);

            coord_a
                .0
                .partial_cmp(&coord_b.0)
                .unwrap_or(Ordering::Equal)
                .then_with(|| coord_a.1.partial_cmp(&coord_b.1).unwrap_or(Ordering::Equal))
        });

        // Wähle Hosts mit guter räumlicher Verteilung
        let mut selected_hosts = Vec::new();
        if groups_needed == 1 {
            selected_hosts.push(available_hosts[0]);
        } else {
            let step = available_hosts.len() / groups_needed;
            for i in 0..groups_needed {
                let idx = (i * step).min(available_hosts.len() - 1);
                selected_hosts.push(available_hosts[idx]);
            }
        }

        Ok(selected_hosts)
    }

    fn find_best_guest_greedy(
        &self,
        host_id: Uuid,
        candidates: &[Uuid],
        meetings: &HashMap<Uuid, HashSet<Uuid>>,
        current_group: &[Uuid],
    ) -> Option<Uuid> {
        let host_meetings = meetings.get(&host_id).unwrap();

        // Bewertung basierend auf mehreren Kriterien
        let mut best_candidate = None;
        let mut best_score = f64::INFINITY;

        for &candidate in candidates {
            let mut score = 0.0;

            // Penalty für vorherige Begegnungen mit Host
            if host_meetings.contains(&candidate) {
                score += 1000.0;
            }

            // Penalty für vorherige Begegnungen mit anderen Gruppenmitgliedern
            let candidate_meetings = meetings.get(&candidate).unwrap();
            for &group_member in current_group {
                if candidate_meetings.contains(&group_member) {
                    score += 500.0;
                }
            }

            // Distanzbonus (kleinere Distanz = bessere Score)
            let host_contact = self.contacts.iter().find(|c| c.id == host_id).unwrap();
            let candidate_contact = self.contacts.iter().find(|c| c.id == candidate).unwrap();

            let distance = self.calculate_distance(
                (
                    host_contact.address.latitude,
                    host_contact.address.longitude,
                ),
                (
                    candidate_contact.address.latitude,
                    candidate_contact.address.longitude,
                ),
            );
            score += distance * 0.1; // Geringere Gewichtung für Distanz

            if score < best_score {
                best_score = score;
                best_candidate = Some(candidate);
            }
        }

        best_candidate
    }

    fn distribute_remaining_participants(
        &self,
        remaining: &[Uuid],
        state: &mut PlanningState,
        course_id: &Uuid,
    ) {
        let mut course_hostings: Vec<&mut HostingData> = state
            .hosting_list
            .iter_mut()
            .filter(|h| h.name == *course_id)
            .collect();

        if course_hostings.is_empty() {
            return;
        }

        // Sortiere Hostings nach aktueller Größe (kleinste zuerst)
        course_hostings.sort_by_key(|h| h.guest_list.len());

        let mut hosting_idx = 0;
        for &participant_id in remaining {
            let hosting = &mut course_hostings[hosting_idx];

            if hosting.guest_list.len() < self.max_group_size - 1 {
                hosting.guest_list.push(participant_id);

                // Aktualisiere Meetings
                let mut group = vec![hosting.host];
                group.extend(&hosting.guest_list);
                for &p1 in &group {
                    for &p2 in &group {
                        if p1 != p2 {
                            state.participant_meetings.get_mut(&p1).unwrap().insert(p2);
                        }
                    }
                }
            }

            hosting_idx = (hosting_idx + 1) % course_hostings.len();
        }
    }

    fn update_meetings(&self, group: &[Uuid], state: &mut PlanningState) {
        for &p1 in group {
            for &p2 in group {
                if p1 != p2 {
                    state.participant_meetings.get_mut(&p1).unwrap().insert(p2);
                }
            }
        }
    }

    fn generate_walking_paths(&self, hosting_list: &[HostingData]) -> HashMap<Uuid, Vec<Uuid>> {
        let mut walking_paths = HashMap::new();

        // Gruppiere Hostings nach Kursen
        let mut course_hostings: HashMap<Uuid, Vec<&HostingData>> = HashMap::new();
        for hosting in hosting_list {
            course_hostings
                .entry(hosting.name)
                .or_default()
                .push(hosting);
        }

        // Für jeden Kontakt, erstelle den Pfad durch die Kurse
        for contact in &self.contacts {
            let mut path = Vec::new();

            // Sortiere Kurse nach Zeit
            let mut sorted_courses = self.courses.clone();
            sorted_courses.sort_by_key(|c| c.time);

            for course in &sorted_courses {
                if let Some(hostings) = course_hostings.get(&course.id) {
                    // Finde das Hosting wo dieser Kontakt ist
                    for hosting in hostings {
                        if hosting.host == contact.id || hosting.guest_list.contains(&contact.id) {
                            path.push(hosting.id);
                            break;
                        }
                    }
                }
            }

            walking_paths.insert(contact.id, path);
        }

        walking_paths
    }

    fn local_search_improve(&self, mut plan: CookAndRunPlan) -> Result<CookAndRunPlan, String> {
        let max_iterations = 100;
        let mut current_violations = plan.meeting_violations;

        for iteration in 0..max_iterations {
            if current_violations == 0 {
                break;
            }

            let mut improved = false;

            // Versuche Teilnehmer zwischen Gruppen zu tauschen
            for course in &self.courses {
                let mut course_hostings: Vec<usize> = plan
                    .hosting_list
                    .iter()
                    .enumerate()
                    .filter(|(_, h)| h.name == course.id)
                    .map(|(i, _)| i)
                    .collect();

                for i in 0..course_hostings.len() {
                    for j in (i + 1)..course_hostings.len() {
                        let idx1 = course_hostings[i];
                        let idx2 = course_hostings[j];

                        // Versuche Gäste zwischen den Gruppen zu tauschen
                        if !plan.hosting_list[idx1].guest_list.is_empty()
                            && !plan.hosting_list[idx2].guest_list.is_empty()
                        {
                            let guest1 = plan.hosting_list[idx1].guest_list[0];
                            let guest2 = plan.hosting_list[idx2].guest_list[0];

                            // Simuliere Tausch
                            let mut new_hosting_list = plan.hosting_list.clone();
                            new_hosting_list[idx1].guest_list[0] = guest2;
                            new_hosting_list[idx2].guest_list[0] = guest1;

                            let new_violations = self.count_meeting_violations(&new_hosting_list);
                            if new_violations < current_violations {
                                plan.hosting_list = new_hosting_list;
                                current_violations = new_violations;
                                improved = true;
                                break;
                            }
                        }
                    }
                    if improved {
                        break;
                    }
                }
                if improved {
                    break;
                }
            }

            if !improved {
                break;
            }

            if iteration % 10 == 0 {
                println!("Iteration {}: {} Violations", iteration, current_violations);
            }
        }

        plan.meeting_violations = current_violations;
        plan.total_distance = self.calculate_total_distance(&plan.hosting_list);
        plan.walking_paths = self.generate_walking_paths(&plan.hosting_list);

        Ok(plan)
    }

    fn calculate_total_distance(&self, hosting_list: &[HostingData]) -> f64 {
        let mut total_distance = 0.0;
        let contact_map: HashMap<Uuid, &ContactData> =
            self.contacts.iter().map(|c| (c.id, c)).collect();

        for hosting in hosting_list {
            if let Some(host) = contact_map.get(&hosting.host) {
                for &guest_id in &hosting.guest_list {
                    if let Some(guest) = contact_map.get(&guest_id) {
                        let distance = self.calculate_distance(
                            (host.address.latitude, host.address.longitude),
                            (guest.address.latitude, guest.address.longitude),
                        );
                        total_distance += distance;
                    }
                }
            }
        }

        total_distance
    }

    fn calculate_distance(&self, coord1: (f64, f64), coord2: (f64, f64)) -> f64 {
        // Haversine-Formel für Koordinaten in Grad (lat/lon)
        let r = 6371.0; // Erdradius in km

        let lat1_rad = coord1.0.to_radians();
        let lat2_rad = coord2.0.to_radians();
        let delta_lat = (coord2.0 - coord1.0).to_radians();
        let delta_lon = (coord2.1 - coord1.1).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        r * c
    }

    fn count_meeting_violations(&self, hosting_list: &[HostingData]) -> usize {
        let mut all_meetings: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();
        let mut violations = 0;

        for contact in &self.contacts {
            all_meetings.insert(contact.id, HashSet::new());
        }

        for hosting in hosting_list {
            let mut group = vec![hosting.host];
            group.extend(&hosting.guest_list);

            for &p1 in &group {
                for &p2 in &group {
                    if p1 != p2 {
                        if all_meetings.get(&p1).unwrap().contains(&p2) {
                            violations += 1;
                        }
                        all_meetings.get_mut(&p1).unwrap().insert(p2);
                    }
                }
            }
        }

        violations / 2 // Teile durch 2, da jede Begegnung doppelt gezählt wird
    }

    pub fn print_plan(&self, plan_data: &PlanData) {
        println!("=== Cook and Run Plan ===");
        println!("Gesamtdistanz: {:.2} km", plan_data.greatest_distance);
        println!();

        let contact_map: HashMap<Uuid, &ContactData> =
            self.contacts.iter().map(|c| (c.id, c)).collect();
        let course_map: HashMap<Uuid, &CourseData> =
            self.courses.iter().map(|c| (c.id, c)).collect();

        for course in &self.courses {
            println!("=== {} ({}) ===", course.name, course.time.format("%H:%M"));
            let course_hostings: Vec<_> = plan_data
                .hosting_list
                .iter()
                .filter(|h| h.name == course.id)
                .collect();

            for (i, hosting) in course_hostings.iter().enumerate() {
                let host = contact_map.get(&hosting.host).unwrap();

                println!(
                    "Gruppe {}: Host: {} ({})",
                    i + 1,
                    host.team_name,
                    host.address.address
                );
                print!("  Gäste: ");

                for (j, &guest_id) in hosting.guest_list.iter().enumerate() {
                    let guest = contact_map.get(&guest_id).unwrap();
                    if j > 0 {
                        print!(", ");
                    }
                    print!("{}", guest.team_name);
                }

                println!();
                println!("  Gruppengröße: {}", hosting.guest_list.len() + 1);
            }
            println!();
        }
    }

    pub fn validate_plan(&self, plan_data: &PlanData) -> Result<(), String> {
        // Validiere dass jeder Kontakt in jedem Kurs genau einmal vorkommt
        for course in &self.courses {
            let mut contacts_in_course = HashSet::new();

            for hosting in &plan_data.hosting_list {
                if hosting.name == course.id {
                    contacts_in_course.insert(hosting.host);
                    for &guest in &hosting.guest_list {
                        if !contacts_in_course.insert(guest) {
                            return Err(format!(
                                "Kontakt {:?} ist mehrfach in Kurs {} eingeteilt",
                                guest, course.name
                            ));
                        }
                    }
                }
            }

            if contacts_in_course.len() != self.contacts.len() {
                return Err(format!(
                    "Nicht alle Kontakte sind in Kurs {} eingeteilt",
                    course.name
                ));
            }
        }

        // Validiere dass jeder Kontakt maximal einmal hostet
        let mut host_count = HashMap::new();
        for hosting in &plan_data.hosting_list {
            *host_count.entry(hosting.host).or_insert(0) += 1;
        }

        for (host_id, count) in host_count {
            if count > 1 {
                let host = self.contacts.iter().find(|c| c.id == host_id).unwrap();
                return Err(format!(
                    "Kontakt {} hostet {} mal (maximal 1 mal erlaubt)",
                    host.team_name, count
                ));
            }
        }

        Ok(())
    }
}

// Hauptfunktion für die Integration
pub fn generate_cook_and_run_plan(mut cook_and_run_data: CookAndRunData) -> Option<PlanData> {
    let solver = CookAndRunSolver::new(
        cook_and_run_data.contact_list.clone(),
        cook_and_run_data.course_list.clone(),
        cook_and_run_data.course_with_more_hosts,
    );

    solver.get_plan()
}
