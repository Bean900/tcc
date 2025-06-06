use uuid::Uuid;
use web_sys::console;

use super::{CookAndRunData, CookAndRunMinimalData, StorageR, StorageW};

const DATA_KEY: &str = "tcc_data";

#[derive(PartialEq, Clone)]
pub struct LocalStorage {
    storage: web_sys::Storage,
    stored_data: Vec<CookAndRunData>,
}

impl LocalStorage {
    pub fn new() -> Result<Self, String> {
        console::log_1(&format!("LocalStorage - Creating local storage instance").into());
        let window = web_sys::window();
        if window.is_none() {
            return Err("No global `window` exists".to_string());
        }
        let window = window.expect("Expected a window");

        let storage = window.session_storage();
        if storage.is_err() {
            let error = storage
                .err()
                .expect("Expected session storage error")
                .as_string()
                .expect("Expected session storage error to be string");
            console::error_1(&format!("LocalStorage - Storage error: {}", error).into());
            return Err(format!("Session storage could not be loaded: {}", error));
        }

        let storage = storage.expect("Expected session storage");
        if storage.is_none() {
            console::error_1(&format!("LocalStorage - Error: Session storage is not set!").into());
            return Err("Session storage is not set".to_string());
        }

        let storage = storage.expect("Expected session storage");
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
            console::log_1(&format!("LocalStorage - New storage created!").into());
            return Ok(LocalStorage {
                storage,
                stored_data: vec![],
            });
        }
        let data = data.expect("Expected data to be set");
        let stored_data: Result<Vec<CookAndRunData>, serde_json::Error> =
            serde_json::from_str(&data);
        if stored_data.is_err() {
            let error = stored_data.err().expect("Expected serde error");
            console::error_1(&format!("LocalStorage - Parsing error: {}", error).into());
            return Err(format!("Data could not parse json: {}", error));
        }
        let stored_data = stored_data.expect("Expected parsed data");
        console::log_1(&format!("LocalStorage - Existing storage connected!").into());
        Ok(LocalStorage {
            storage,
            stored_data,
        })
    }
}

impl StorageR for LocalStorage {
    fn select_all_cook_and_run_minimal(&self) -> Result<Vec<CookAndRunMinimalData>, String> {
        console::log_1(&format!("LocalStorage - Load cook and run minimal!").into());
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
        console::log_1(&format!("LocalStorage - Create cook and run \"{}\"!", name).into());
        let cook_and_run = CookAndRunData::new(uuid, name);
        self.stored_data.push(cook_and_run.clone());

        let stored_data_string = serde_json::to_string(&self.stored_data);

        if stored_data_string.is_err() {
            self.stored_data.pop();
            let error = stored_data_string.err().expect("Expected serde error");
            console::error_1(
                &format!(
                    "LocalStorage - Struct could not be parse into json: {}",
                    error
                )
                .into(),
            );
            return Err(format!("Struct could not be parse into json: {}", error));
        }

        let stored_data_string = stored_data_string.expect("Expected parsed data");

        let result = self.storage.set_item(DATA_KEY, &stored_data_string);

        if result.is_err() {
            self.stored_data.pop();
            let error = result
                .err()
                .expect("Expected storage error")
                .as_string()
                .expect("Expected storage error to be string");
            console::error_1(&format!("LocalStorage - Data could not be stored: {}", error).into());
            return Err(format!("Data could not be stored: {}", error));
        }

        console::log_1(&format!("LocalStorage - Created cook and run!").into());
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

    fn create_team_note_in_cook_and_run(
        &mut self,
        id: Uuid,
        team_id: Uuid,
        headline: String,
        description: String,
    ) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                if let Some(team) = data.contact_list.iter_mut().find(|x| x.id == team_id) {
                    let note = super::NoteData {
                        id: Uuid::new_v4(),
                        headline,
                        description,
                        created: chrono::Utc::now(),
                    };
                    team.notes.push(note);
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
        for data in &mut self.stored_data {
            if data.id == id {
                if let Some(team) = data.contact_list.iter_mut().find(|x| x.id == team_id) {
                    team.needs_check = needs_check;
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
                    return Err(format!("Team with ID {} not found", team_id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn delete_team_in_cook_and_run(&mut self, id: Uuid, team_id: Uuid) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                if let Some(index) = data.contact_list.iter().position(|x| x.id == team_id) {
                    data.contact_list.remove(index);
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
        for data in &mut self.stored_data {
            if data.id == id {
                data.start_point = start_point;
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

    fn update_goal_point_in_cook_and_run(
        &mut self,
        id: Uuid,
        end_point: Option<super::MeetingPointData>,
    ) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                data.end_point = end_point;
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

    fn add_course_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data: super::CourseData,
    ) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                data.course_list.push(course_data);
                let stored_data_string = serde_json::to_string(&self.stored_data);

                if stored_data_string.is_err() {
                    return Err(format!(
                        "Struct could not be parsed into json: {}",
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

    fn update_course_in_cook_and_run(
        &mut self,
        id: Uuid,
        course_data: super::CourseData,
    ) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                if let Some(index) = data.course_list.iter().position(|x| x.id == course_data.id) {
                    data.course_list[index] = course_data;
                    let stored_data_string = serde_json::to_string(&self.stored_data);

                    if stored_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parsed into json: {}",
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
                    return Err(format!("Course with ID {} not found", course_data.id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }

    fn delete_course_in_cook_and_run(&mut self, id: Uuid, course_id: Uuid) -> Result<(), String> {
        for data in &mut self.stored_data {
            if data.id == id {
                if let Some(index) = data.course_list.iter().position(|x| x.id == course_id) {
                    data.course_list.remove(index);
                    let stored_data_string = serde_json::to_string(&self.stored_data);

                    if stored_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parsed into json: {}",
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
        for data in &mut self.stored_data {
            if data.id == id {
                if data
                    .course_list
                    .iter_mut()
                    .find(|x| x.id == course_data_id)
                    .is_some()
                {
                    data.course_with_more_hosts = Some(course_data_id);
                    let stored_data_string = serde_json::to_string(&self.stored_data);

                    if stored_data_string.is_err() {
                        return Err(format!(
                            "Struct could not be parsed into json: {}",
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
                    return Err(format!("Course with ID {} not found", course_data_id));
                }
            }
        }
        Err(format!("Cook and run project with ID {} not found", id))
    }
}
