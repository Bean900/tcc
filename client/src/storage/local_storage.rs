use uuid::Uuid;

use super::{CookAndRunData, CookAndRunMinimalData, StorageR, StorageW};

const DATA_KEY: &str = "tcc_data";

#[derive(PartialEq, Clone)]
pub struct LocalStorage {
    storage: web_sys::Storage,
    stored_data: Vec<CookAndRunData>,
}

impl LocalStorage {
    pub fn new() -> Result<Self, String> {
        let window = web_sys::window();
        if window.is_none() {
            return Err("No global `window` exists".to_string());
        }
        let window = window.expect("Expected a window");

        let storage = window.session_storage();
        if storage.is_err() {
            return Err(format!(
                "Session storage could not be loaded: {}",
                storage
                    .err()
                    .expect("Expected session storage error")
                    .as_string()
                    .expect("Expected session storage error to be string")
            ));
        }

        let storage = storage.expect("Expected session storage");
        if storage.is_none() {
            return Err("Session storage is not set".to_string());
        }

        let storage = storage.expect("Expected session storage");
        let data = storage.get_item(DATA_KEY);

        if data.is_err() {
            return Err(format!(
                "Data could not be loaded: {}",
                data.err()
                    .expect("Expected data error")
                    .as_string()
                    .expect("Expected data error to be string")
            ));
        }
        let data = data.expect("Expected no data error");
        if data.is_none() {
            return Ok(LocalStorage {
                storage,
                stored_data: vec![],
            });
        }
        let data = data.expect("Expected data to be set");
        let stored_data: Result<Vec<CookAndRunData>, serde_json::Error> =
            serde_json::from_str(&data);
        if stored_data.is_err() {
            return Err(format!(
                "Data could not parse json: {}",
                stored_data.err().expect("Expected serde error")
            ));
        }
        let stored_data = stored_data.expect("Expected parsed data");
        Ok(LocalStorage {
            storage,
            stored_data,
        })
    }
}

impl StorageR for LocalStorage {
    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimalData>, String> {
        Ok(self.stored_data.iter().map(|x| x.to_minimal()).collect())
    }
    fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String> {
        for data in &self.stored_data {
            if data.id == id {
                return Ok(data.clone());
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }
}

impl StorageW for LocalStorage {
    fn create_cook_and_run(&mut self, uuid: Uuid, name: String) -> Result<(), String> {
        let cook_and_run = CookAndRunData::new(uuid, name);
        self.stored_data.push(cook_and_run.clone());

        let stored_data_string = serde_json::to_string(&self.stored_data);

        if stored_data_string.is_err() {
            self.stored_data.pop();
            return Err(format!(
                "Struct could not be parse into json: {}",
                stored_data_string.err().expect("Expected serde error")
            ));
        }

        let stored_data_string = stored_data_string.expect("Expected parsed data");

        let result = self.storage.set_item(DATA_KEY, &stored_data_string);

        if result.is_err() {
            self.stored_data.pop();
            return Err(format!(
                "Data could not be stored: {}",
                result
                    .err()
                    .expect("Expected storage error")
                    .as_string()
                    .expect("Expected storage error to be string")
            ));
        }

        Ok(())
    }

    fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String> {
        let index = self.stored_data.iter().position(|x| x.id == id);
        if index.is_none() {
            return Ok(()); // Nothing to delete, return early
        }
        let index = index.expect("Expected index to be set");
        self.stored_data.remove(index);

        let stored_data_string = serde_json::to_string(&self.stored_data);

        if stored_data_string.is_err() {
            return Err(format!(
                "Struct could not be parse into json: {}",
                stored_data_string.err().expect("Expected serde error")
            ));
        }

        let stored_data_string = stored_data_string.expect("Expected parsed data");

        let result = self.storage.set_item(DATA_KEY, &stored_data_string);

        if result.is_err() {
            return Err(format!(
                "Data could not be stored: {}",
                result
                    .err()
                    .expect("Expected storage error")
                    .as_string()
                    .expect("Expected storage error to be string")
            ));
        }

        Ok(())
    }

    fn rename_cook_and_run(&mut self, id: Uuid, new_name: String) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                data.name = new_name;
                let stored_data_string = serde_json::to_string(&self.stored_data);

                if stored_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parse into json: {}",
                        stored_data_string.err().expect("Expected serde error")
                    ));
                }

                let stored_data_string = stored_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &stored_data_string);

                if result.is_err() {
                    return Err(format!(
                        "Data could not be stored: {}",
                        result
                            .err()
                            .expect("Expected storage error")
                            .as_string()
                            .expect("Expected storage error to be string")
                    ));
                }
                return Ok(());
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn add_team_to_cook_and_run(
        &mut self,
        id: Uuid,
        team: super::ContactData,
    ) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                data.contact_list.push(team);
                let stored_data_string = serde_json::to_string(&self.stored_data);

                if stored_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parse into json: {}",
                        stored_data_string.err().expect("Expected serde error")
                    ));
                }

                let stored_data_string = stored_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &stored_data_string);

                if result.is_err() {
                    return Err(format!(
                        "Data could not be stored: {}",
                        result
                            .err()
                            .expect("Expected storage error")
                            .as_string()
                            .expect("Expected storage error to be string")
                    ));
                }
                return Ok(());
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn update_team_in_cook_and_run(
        &mut self,
        id: Uuid,
        team: super::ContactData,
    ) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                if let Some(index) = data.contact_list.iter().position(|x| x.id == team.id) {
                    data.contact_list[index] = team;
                    let stored_data_string = serde_json::to_string(&self.stored_data);

                    if stored_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parse into json: {}",
                            stored_data_string.err().expect("Expected serde error")
                        ));
                    }

                    let stored_data_string = stored_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &stored_data_string);

                    if result.is_err() {
                        return Err(format!(
                            "Data could not be stored: {}",
                            result
                                .err()
                                .expect("Expected storage error")
                                .as_string()
                                .expect("Expected storage error to be string")
                        ));
                    }
                    return Ok(());
                } else {
                    return Err(format!("Team with ID {} not found", team.id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }
}
