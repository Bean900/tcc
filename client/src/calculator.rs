use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use uuid::Uuid;

use crate::storage::{AddressData, ContactData, CookAndRunData, HostingData, PlanData};

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
pub struct Plan {
    pub id: Uuid,
    pub hosting_list: HashMap<Uuid /*Hosting ID */, HostingData>,
    pub walking_path: HashMap<Uuid /*Contact ID */, Vec<Uuid /*Hosting ID */>>,
    pub greatest_distance: f64,
}

impl Calculator {
    pub fn new(cook_and_run_data: &CookAndRunData) -> Result<Calculator, String> {
        let calc = Calculator {
            contact_list: cook_and_run_data
                .contact_list
                .iter()
                .map(|c| (c.id, c.clone()))
                .collect(),
            course_list: cook_and_run_data.course_list.iter().map(|c| c.id).collect(),
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
        let top_plan = Arc::clone(&self.top_plan);
        let should_stop = Arc::clone(&self.should_stop);

        // Setze should_stop zurück
        *should_stop.lock().unwrap() = false;

        thread::spawn(move || {
            let mut best_plan: Option<PlanData> = None;
            let mut best_fitness = f64::MAX;

            // Generiere und bewerte verschiedene Lösungen
            while !*should_stop.lock().unwrap() {
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    pub fn stop(&self) {
        *self.should_stop.lock().unwrap() = true;
    }

    pub fn get_top_plan(&self) -> Option<PlanData> {
        todo!("self.top_plan.lock().unwrap().clone()");
    }

    fn calculate_fitness(&self, plan: &Plan) -> f64 {
        let mut fitness = 0.0;

        for (_, hosting_path) in plan.walking_path.iter() {
            let mut current_fitness = 0.0;

            let mut hosting_iter = hosting_path.iter();
            let mut last_addr = self.get_address(
                &plan
                    .hosting_list
                    .get(hosting_iter.next().expect("Expect first Hosting"))
                    .expect("Expect to find Hosting")
                    .host,
            );

            if let Some(start_point) = self.start_point.as_ref() {
                current_fitness = start_point.distance(last_addr);
            }

            loop {
                let next_hosting = hosting_iter.next();
                if let Some(next_hosting) = next_hosting {
                    let next_addr = self.get_address(
                        &plan
                            .hosting_list
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

            if let Some(end_point) = self.end_point.as_ref() {
                current_fitness = current_fitness + end_point.distance(last_addr);
            }

            if current_fitness > fitness {
                fitness = current_fitness;
            }
        }

        fitness
    }

    fn get_address(&self, contact_id: &Uuid) -> &AddressData {
        &self
            .contact_list
            .get(contact_id)
            .expect("Expect to find Contact")
            .address
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
