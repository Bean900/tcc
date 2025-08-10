use chrono::NaiveDate;
use uuid::Uuid;
use web_sys::console;

use crate::storage::AuthData;

use super::{CookAndRunData, CookAndRunMinimalData};

use crate::storage::Storage;

const DATA_KEY: &str = "tcc_data";
const AUTH_KEY: &str = "auth_data";

#[derive(PartialEq, Clone)]
pub struct LocalStorage {
    storage: web_sys::Storage,
    cook_and_run_data: Vec<CookAndRunData>,
    auth_data: AuthData,
}

impl LocalStorage {
    pub fn new() -> Result<Self, String> {
        let window = web_sys::window().ok_or_else(|| "No global `window` exists".to_string())?;

        let storage = window
            .session_storage()
            .map_err(|e| {
                format!(
                    "Session storage could not be loaded: {}",
                    e.as_string().unwrap_or_default()
                )
            })?
            .ok_or_else(|| "Session storage is not available".to_string())?;

        let cook_and_run_data = Self::get_cook_and_run_data(storage.clone())
            .map_err(|e| format!("Cook and run data could not be loaded: {}", e))?;

        let auth_data = Self::get_auth_data(storage.clone());
        if auth_data.is_err() {
            let error = auth_data.err().expect("Expected data error");
            console::error_1(&format!("LocalStorage - Auth data error: {}", error).into());
            return Err(format!("Auth data could not be loaded: {}", error));
        }

        let auth_data = auth_data.expect("Expected stored data");

        Ok(LocalStorage {
            storage,
            cook_and_run_data,
            auth_data,
        })
    }

    fn get_cook_and_run_data_by_id(&self, id: Uuid) -> Option<CookAndRunData> {
        for data in &self.cook_and_run_data {
            if data.id == id {
                return Some(data.clone());
            }
        }
        None
    }

    fn update_cook_and_run_data(&mut self, cook_and_run: &CookAndRunData) -> Result<(), String> {
        let mut found = false;
        let mut new_data = self.cook_and_run_data.clone();
        for data in &mut new_data {
            if data.id == cook_and_run.id {
                *data = cook_and_run.clone();
                found = true;
                break;
            }
        }
        if !found {
            return Err(format!(
                "Cook and run project with ID {} not found",
                cook_and_run.id
            ));
        }
        let cook_and_run_data_string = serde_json::to_string(&new_data)
            .map_err(|e| format!("Struct could not be parsed into json: {}", e))?;

        self.storage
            .set_item(DATA_KEY, &cook_and_run_data_string)
            .map_err(|e| {
                format!(
                    "Data could not be stored: {}",
                    e.as_string().unwrap_or_default()
                )
            })?;
        self.cook_and_run_data = new_data;

        Ok(())
    }

    fn create_cook_and_run_data(&mut self, cook_and_run: &CookAndRunData) -> Result<(), String> {
        let mut new_data = self.cook_and_run_data.clone();
        for data in &mut new_data {
            if data.id == cook_and_run.id {
                return Err(format!(
                    "Cook and run project with ID {} already exists",
                    cook_and_run.id
                ));
            }
        }

        new_data.push(cook_and_run.clone());

        let cook_and_run_data_string = serde_json::to_string(&new_data)
            .map_err(|e| format!("Struct could not be parsed into json: {}", e))?;

        self.storage
            .set_item(DATA_KEY, &cook_and_run_data_string)
            .map_err(|e| {
                format!(
                    "Data could not be stored: {}",
                    e.as_string().unwrap_or_default()
                )
            })?;
        self.cook_and_run_data = new_data;

        Ok(())
    }

    fn delete_cook_and_run_data(&mut self, cook_and_run_id: Uuid) -> Result<(), String> {
        let mut new_data = self.cook_and_run_data.clone();
        let original_len = new_data.len();
        new_data.retain(|data| data.id != cook_and_run_id);

        if new_data.len() == original_len {
            return Err(format!(
                "Cook and run project with ID {} not found",
                cook_and_run_id
            ));
        }

        let cook_and_run_data_string = serde_json::to_string(&new_data)
            .map_err(|e| format!("Struct could not be parsed into json: {}", e))?;

        self.storage
            .set_item(DATA_KEY, &cook_and_run_data_string)
            .map_err(|e| {
                format!(
                    "Data could not be stored: {}",
                    e.as_string().unwrap_or_default()
                )
            })?;

        self.cook_and_run_data = new_data;

        Ok(())
    }

    fn get_auth_data(storage: Storage) -> Result<AuthData, String> {
        let data = storage.get_item(AUTH_KEY);
        if data.is_err() {
            let error = data
                .err()
                .expect("Expected auth data error")
                .as_string()
                .expect("Expected auth data error to be string");
            console::error_1(&format!("LocalStorage - Auth data error: {}", error).into());
            return Err(format!("Auth data could not be loaded: {}", error));
        }
        let data = data.expect("Expected no auth data error");
        if data.is_none() {
            return Ok(AuthData {
                session_data: None,
                process_data: None,
            });
        }
        let data = data.expect("Expected auth data to be set");
        let auth_data: Result<AuthData, serde_json::Error> = serde_json::from_str(&data);
        if auth_data.is_err() {
            let error = auth_data.err().expect("Expected serde error");
            console::error_1(&format!("LocalStorage - Auth data parsing error: {}", error).into());
            return Err(format!("Auth data could not parse json: {}", error));
        }
        let auth_data = auth_data.expect("Expected parsed auth data");
        console::log_1(&format!("LocalStorage - Auth data parsed").into());
        Ok(auth_data)
    }

    fn get_cook_and_run_data(storage: Storage) -> Result<Vec<CookAndRunData>, String> {
        let data = storage.get_item(DATA_KEY);

        if data.is_err() {
            let error = data
                .err()
                .expect("Expected data error")
                .as_string()
                .expect("Expected data error to be string");
            console::error_1(&format!("LocalStorage - Data error: {}", error).into());
            return Err(format!("Data could not be loaded: {}", error));
        }

        let data = data.expect("Expected no data error");
        if data.is_none() {
            return Ok(vec![]);
        }

        let data = data.expect("Expected data to be set");
        let cook_and_run_data: Result<Vec<CookAndRunData>, serde_json::Error> =
            serde_json::from_str(&data);
        if cook_and_run_data.is_err() {
            let error = cook_and_run_data.err().expect("Expected serde error");
            console::error_1(&format!("LocalStorage - Data parsing error: {}", error).into());
            return Err(format!("Data could not parse json: {}", error));
        }
        let cook_and_run_data = cook_and_run_data.expect("Expected parsed data");
        console::log_1(&format!("LocalStorage - Data parsed").into());
        return Ok(cook_and_run_data);
    }

    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimalData>, String> {
        console::log_1(&format!("LocalStorage - Load cook and run minimal!").into());
        Ok(self
            .cook_and_run_data
            .iter()
            .map(|x| x.to_minimal())
            .collect())
    }
    fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String> {
        for data in &self.cook_and_run_data {
            if data.id == id {
                return Ok(data.clone());
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn select_cook_and_run_json(&self, id: Uuid) -> Result<String, String> {
        console::log_1(&format!("LocalStorage - Load cook and run JSON!").into());
        for data in &self.cook_and_run_data {
            if data.id == id {
                let json_string = serde_json::to_string(data);
                if json_string.is_err() {
                    let error = json_string.err().expect("Expected serde error");
                    console::error_1(
                        &format!("LocalStorage - JSON parsing error: {}", error).into(),
                    );
                    return Err(format!(
                        "Cook and run project could not be parsed into JSON: {}",
                        error
                    ));
                }
                return Ok(json_string.expect("Expected parsed JSON"));
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn select_auth_data(&self) -> Result<AuthData, String> {
        console::log_1(&format!("LocalStorage - Load auth data!").into());
        Ok(self.auth_data.clone())
    }

    fn update_meta_of_cook_and_run(
        &mut self,
        id: Uuid,
        new_name: String,
        new_plan_text: Option<String>,
        occur: NaiveDate,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                data.name = new_name;
                data.plan_text = new_plan_text;
                data.occur = occur;
                let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                if cook_and_run_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parse into json: {}",
                        cook_and_run_data_string
                            .err()
                            .expect("Expected serde error")
                    ));
                }

                let cook_and_run_data_string =
                    cook_and_run_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                data.contact_list.push(team);
                let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                if cook_and_run_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parse into json: {}",
                        cook_and_run_data_string
                            .err()
                            .expect("Expected serde error")
                    ));
                }

                let cook_and_run_data_string =
                    cook_and_run_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                if let Some(index) = data.contact_list.iter().position(|x| x.id == team.id) {
                    data.contact_list[index] = team;
                    let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                    if cook_and_run_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parse into json: {}",
                            cook_and_run_data_string
                                .err()
                                .expect("Expected serde error")
                        ));
                    }

                    let cook_and_run_data_string =
                        cook_and_run_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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

    fn create_team_note_in_cook_and_run(
        &mut self,
        id: Uuid,
        team_id: Uuid,
        headline: String,
        description: String,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                if let Some(team) = data.contact_list.iter_mut().find(|x| x.id == team_id) {
                    let note = super::NoteData {
                        id: Uuid::new_v4(),
                        headline,
                        description,
                        created: chrono::Utc::now(),
                    };
                    team.notes.push(note);
                    let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                    if cook_and_run_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parse into json: {}",
                            cook_and_run_data_string
                                .err()
                                .expect("Expected serde error")
                        ));
                    }

                    let cook_and_run_data_string =
                        cook_and_run_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
                    return Err(format!("Team with ID {} not found", team_id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn update_team_needs_ckeck_in_cook_and_run(
        &mut self,
        id: Uuid,
        team_id: Uuid,
        needs_check: bool,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                if let Some(team) = data.contact_list.iter_mut().find(|x| x.id == team_id) {
                    team.needs_check = needs_check;
                    let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                    if cook_and_run_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parse into json: {}",
                            cook_and_run_data_string
                                .err()
                                .expect("Expected serde error")
                        ));
                    }

                    let cook_and_run_data_string =
                        cook_and_run_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
                    return Err(format!("Team with ID {} not found", team_id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn delete_team_in_cook_and_run(&mut self, id: Uuid, team_id: Uuid) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                if let Some(index) = data.contact_list.iter().position(|x| x.id == team_id) {
                    data.contact_list.remove(index);
                    let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                    if cook_and_run_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parse into json: {}",
                            cook_and_run_data_string
                                .err()
                                .expect("Expected serde error")
                        ));
                    }

                    let cook_and_run_data_string =
                        cook_and_run_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
                    return Err(format!("Team with ID {} not found", team_id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn update_start_point_in_cook_and_run(
        &mut self,
        id: Uuid,
        start_point: Option<super::MeetingPointData>,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                data.start_point = start_point;
                let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                if cook_and_run_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parse into json: {}",
                        cook_and_run_data_string
                            .err()
                            .expect("Expected serde error")
                    ));
                }

                let cook_and_run_data_string =
                    cook_and_run_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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

    fn update_goal_point_in_cook_and_run(
        &mut self,
        id: Uuid,
        end_point: Option<super::MeetingPointData>,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                data.end_point = end_point;
                let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                if cook_and_run_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parse into json: {}",
                        cook_and_run_data_string
                            .err()
                            .expect("Expected serde error")
                    ));
                }

                let cook_and_run_data_string =
                    cook_and_run_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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

    fn add_course_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data: super::CourseData,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                data.course_list.push(course_data);
                let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                if cook_and_run_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parsed into json: {}",
                        cook_and_run_data_string
                            .err()
                            .expect("Expected serde error")
                    ));
                }

                let cook_and_run_data_string =
                    cook_and_run_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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

    fn update_course_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data: super::CourseData,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                if let Some(index) = data.course_list.iter().position(|x| x.id == course_data.id) {
                    data.course_list[index] = course_data;
                    let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                    if cook_and_run_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parsed into json: {}",
                            cook_and_run_data_string
                                .err()
                                .expect("Expected serde error")
                        ));
                    }

                    let cook_and_run_data_string =
                        cook_and_run_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
                    return Err(format!("Course with ID {} not found", course_data.id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn delete_course_in_cook_and_run(&mut self, id: Uuid, course_id: Uuid) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                if let Some(index) = data.course_list.iter().position(|x| x.id == course_id) {
                    data.course_list.remove(index);
                    let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                    if cook_and_run_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parsed into json: {}",
                            cook_and_run_data_string
                                .err()
                                .expect("Expected serde error")
                        ));
                    }

                    let cook_and_run_data_string =
                        cook_and_run_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
                    return Err(format!("Course with ID {} not found", course_id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn update_course_with_more_hosts_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data_id: Uuid,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                if data
                    .course_list
                    .iter_mut()
                    .find(|x| x.id == course_data_id)
                    .is_some()
                {
                    data.course_with_more_hosts = Some(course_data_id);
                    let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                    if cook_and_run_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parsed into json: {}",
                            cook_and_run_data_string
                                .err()
                                .expect("Expected serde error")
                        ));
                    }

                    let cook_and_run_data_string =
                        cook_and_run_data_string.expect("Expected parsed data");

                    let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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
                    return Err(format!("Course with ID {} not found", course_data_id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn update_top_plan_in_cook_and_run(
        &mut self,
        id: Uuid,
        top_plan: Option<super::PlanData>,
    ) -> Result<(), String> {
        for data in &mut self.cook_and_run_data {
            if data.id == id {
                data.top_plan = top_plan;
                let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

                if cook_and_run_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parsed into json: {}",
                        cook_and_run_data_string
                            .err()
                            .expect("Expected serde error")
                    ));
                }

                let cook_and_run_data_string =
                    cook_and_run_data_string.expect("Expected parsed data");

                let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

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

    fn create_cook_and_run_json(&mut self, uuid: Uuid, json: String) -> Result<(), String> {
        console::log_1(&format!("LocalStorage - Create cook and run from JSON!").into());

        for data in &mut self.cook_and_run_data {
            if data.id == uuid {
                console::error_1(
                    &format!(
                        "LocalStorage - Cook and run with ID {} already exists!",
                        uuid
                    )
                    .into(),
                );
                return Err(format!(
                    "Cook and run project with ID {} already exists",
                    uuid
                ));
            }
        }

        let cook_and_run: Result<CookAndRunData, serde_json::Error> = serde_json::from_str(&json);
        if cook_and_run.is_err() {
            let error = cook_and_run.err().expect("Expected serde error");
            console::error_1(&format!("LocalStorage - JSON parsing error: {}", error).into());
            return Err(format!(
                "Cook and run project could not be parsed from JSON: {}",
                error
            ));
        }
        let mut cook_and_run = cook_and_run.expect("Expected parsed CookAndRunData");
        cook_and_run.id = uuid;

        self.cook_and_run_data.push(cook_and_run.clone());

        let cook_and_run_data_string = serde_json::to_string(&self.cook_and_run_data);

        if cook_and_run_data_string.is_err() {
            self.cook_and_run_data.pop();
            let error = cook_and_run_data_string
                .err()
                .expect("Expected serde error");
            console::error_1(
                &format!(
                    "LocalStorage - Struct could not be parse into json: {}",
                    error
                )
                .into(),
            );
            return Err(format!("Struct could not be parse into json: {}", error));
        }

        let cook_and_run_data_string = cook_and_run_data_string.expect("Expected parsed data");

        let result = self.storage.set_item(DATA_KEY, &cook_and_run_data_string);

        if result.is_err() {
            self.cook_and_run_data.pop();
            let error = result
                .err()
                .expect("Expected storage error")
                .as_string()
                .expect("Expected storage error to be string");
            console::error_1(&format!("LocalStorage - Data could not be stored: {}", error).into());
            return Err(format!("Data could not be stored: {}", error));
        }

        console::log_1(&format!("LocalStorage - Created cook and run from JSON!").into());
        Ok(())
    }

    fn insert_auth_data(&mut self, auth_data: AuthData) -> Result<(), String> {
        console::log_1(&format!("LocalStorage - Insert auth data!").into());
        self.auth_data = auth_data;
        let auth_data_string = serde_json::to_string(&self.auth_data);

        if auth_data_string.is_err() {
            let error = auth_data_string.err().expect("Expected serde error");
            console::error_1(
                &format!(
                    "LocalStorage - Auth data could not be parsed into JSON: {}",
                    error
                )
                .into(),
            );
            return Err(format!(
                "Auth data could not be parsed into JSON: {}",
                error
            ));
        }

        let auth_data_string = auth_data_string.expect("Expected parsed auth data");

        let result = self.storage.set_item(AUTH_KEY, &auth_data_string);

        if result.is_err() {
            let error = result
                .err()
                .expect("Expected storage error")
                .as_string()
                .expect("Expected storage error to be string");
            console::error_1(
                &format!("LocalStorage - Auth data could not be stored: {}", error).into(),
            );
            return Err(format!("Auth data could not be stored: {}", error));
        }

        console::log_1(&format!("LocalStorage - Auth data inserted!").into());
        Ok(())
    }
}

impl Storage for LocalStorage {
    fn create_cook_and_run(&mut self, cook_and_run: &CookAndRunData) -> Result<(), String> {
        self.create_cook_and_run_data(cook_and_run)
    }

    fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String> {
        self.delete_cook_and_run_data(id)
    }

    fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String> {
        let result = self.get_cook_and_run_data_by_id(id);
        match result {
            Some(data) => Ok(data),
            None => Err(format!("Cook and run project with ID {} not found", id)),
        }
    }

    fn select_cook_and_run_minimal_list(&self) -> Result<Vec<CookAndRunMinimalData>, String> {
        Ok(self
            .cook_and_run_data
            .iter()
            .map(|x| x.to_minimal())
            .collect())
    }
}
