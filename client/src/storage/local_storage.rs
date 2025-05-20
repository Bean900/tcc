use uuid::Uuid;

use super::{CookAndRun, Storage};
extern crate serde_json;

const DATA_KEY: &str = "tcc_data";

#[derive(PartialEq, Clone)]
pub struct LocalStorage {
    storage: web_sys::Storage,
    stored_data: Vec<CookAndRun>,
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
        let stored_data: Result<Vec<CookAndRun>, serde_json::Error> = serde_json::from_str(&data);
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

impl Storage for LocalStorage {
    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<super::CookAndRunMinimal>, String> {
        Ok(self.stored_data.iter().map(|x| x.to_minimal()).collect())
    }

    fn create_cook_and_run(&mut self, name: String) -> Result<Uuid, String> {
        let cook_and_run = CookAndRun::new(name);
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

        Ok(cook_and_run.id)
    }
}
