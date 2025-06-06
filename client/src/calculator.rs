use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::storage::{AddressData, ContactData, CookAndRunData, CourseData, PlanData};
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

fn calculate_distance(address_one: &AddressData, address_two: &AddressData) -> f64 {
    let radius_erde_km = 6371.0;

    let lat1_rad = address_one.latitude.to_radians();
    let lon1_rad = address_one.longitude.to_radians();
    let lat2_rad = address_two.latitude.to_radians();
    let lon2_rad = address_two.longitude.to_radians();

    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    let a =
        (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    radius_erde_km * c
}

impl Calculator {
    pub fn new(cook_and_run_data: CookAndRunData) -> Result<Self, String> {
        check_data(&cook_and_run_data)?;
        Ok(Calculator {
            cook_and_run_data,
            calculating: Arc::new(Mutex::new(false)),
            top_plan: Arc::new(Mutex::new(None)),
        })
    }

    pub fn get_top_plan(&self) -> Option<PlanData> {
        self.top_plan
            .lock()
            .expect("Failed to lock top_plan")
            .clone()
    }

    pub fn calculate(&mut self) {
        let walking_path = self.berechne_besuchsplan();
    }

    pub fn stop(&mut self) {
        *self
            .calculating
            .lock()
            .expect("Expect calculating to be set!") = false;
    }

    fn berechne_besuchsplan(&self) -> Vec<HashMap<Uuid, Uuid>> {
        let mut global_plan = Vec::new();
        let mut schon_besucht: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();

        for runde in 0..self.cook_and_run_data.course_list.len() {
            let plan =
                finde_permutation_fuer_runde(&self.cook_and_run_data.contact_list, &schon_besucht);
            if plan.is_none() {
                panic!(
                    "Konnte keine gültige Besuchszuordnung für Runde {} finden",
                    runde
                );
            }

            let runden_plan = plan.unwrap();
            for (besucher, ziel) in &runden_plan {
                schon_besucht.entry(*besucher).or_default().insert(*ziel);
            }

            global_plan.push(runden_plan);
        }

        global_plan
    }
}
fn finde_permutation_fuer_runde(
    kontakte: &Vec<ContactData>,
    schon_besucht: &HashMap<Uuid, HashSet<Uuid>>,
) -> Option<HashMap<Uuid, Uuid>> {
    let mut empfaenger_frei: HashSet<Uuid> = kontakte.iter().map(|k| k.id).collect();
    let mut plan = HashMap::new();

    let erfolg = versuche_match(kontakte, 0, &mut plan, &mut empfaenger_frei, schon_besucht);

    if erfolg {
        Some(plan)
    } else {
        None
    }
}

fn versuche_match(
    kontakte: &Vec<ContactData>,
    index: usize,
    plan: &mut HashMap<Uuid, Uuid>,
    empfaenger_frei: &mut HashSet<Uuid>,
    schon_besucht: &HashMap<Uuid, HashSet<Uuid>>,
) -> bool {
    if index >= kontakte.len() {
        return true; // Alle zugewiesen
    }

    let besucher = &kontakte[index];
    let mut kandidaten: Vec<_> = kontakte
        .iter()
        .filter(|ziel| {
            ziel.id != besucher.id
                && empfaenger_frei.contains(&ziel.id)
                && !schon_besucht
                    .get(&besucher.id)
                    .map_or(false, |s| s.contains(&ziel.id))
        })
        .map(|ziel| {
            (
                ziel.id,
                calculate_distance(&besucher.address, &ziel.address),
            )
        })
        .collect();

    kandidaten.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (ziel_id, _) in kandidaten {
        empfaenger_frei.remove(&ziel_id);
        plan.insert(besucher.id, ziel_id);

        if versuche_match(kontakte, index + 1, plan, empfaenger_frei, schon_besucht) {
            return true;
        }

        // Backtrack
        empfaenger_frei.insert(ziel_id);
        plan.remove(&besucher.id);
    }

    false
}
