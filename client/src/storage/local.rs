use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use super::{CookAndRunData, CookAndRunMetaData};

use crate::storage::{
    CookAndRunCreate, CookAndRunMetaUpdate, CourseCreate, CourseData, CourseUpdate,
    MeetingPointData, NoteCreate, PlanData, Storage, TeamCreate, TeamData, TeamUpdate,
};

const DATA_KEY: &str = "tcc_data";

#[derive(PartialEq, Clone, Debug)]
pub struct LocalStorage {
    storage: web_sys::Storage,
    cook_and_run_data: Vec<CookAndRunData>,
}

impl CourseCreate {
    pub fn to_local(&self, course_id: Uuid) -> CourseData {
        CourseData {
            id: course_id,
            name: self.name.clone(),
            time: self.time,
            has_multiple_hosts: self.has_multiple_hosts,
        }
    }
}

impl CourseUpdate {
    pub fn to_local(&self, course_id: Uuid) -> CourseData {
        CourseData {
            id: course_id,
            name: self.name.clone(),
            time: self.time,
            has_multiple_hosts: self.has_multiple_hosts,
        }
    }
}

impl TeamCreate {
    pub fn to_local(&self, team_id: Uuid) -> TeamData {
        TeamData {
            id: team_id,
            name: self.name.clone(),
            created: Utc::now().naive_utc(),
            edited: Utc::now().naive_utc(),
            address: self.address.clone(),
            mail: self.mail.clone(),
            phone: self.phone.clone(),
            members: self.members,
            diets: self.diets.clone(),
            needs_check: self.needs_check,
            note_list: vec![],
        }
    }
}

impl TeamUpdate {
    pub fn to_local(&self, team_id: Uuid, created: NaiveDateTime) -> TeamData {
        TeamData {
            id: team_id,
            name: self.name.clone(),
            created,
            edited: Utc::now().naive_utc(),
            address: self.address.clone(),
            mail: self.mail.clone(),
            phone: self.phone.clone(),
            members: self.members,
            diets: self.diets.clone(),
            needs_check: self.needs_check,
            note_list: vec![],
        }
    }
}

impl CookAndRunCreate {
    fn to_cook_and_run_data(&self, cook_and_run_id: Uuid) -> CookAndRunData {
        CookAndRunData {
            id: cook_and_run_id,
            name: self.name.clone(),
            created: Utc::now().naive_utc(),
            edited: Utc::now().naive_utc(),
            occur: Utc::now().naive_utc(),
            is_in_cloud: false,
            team_list: vec![],
            course_list: vec![],
            course_with_more_hosts: None,
            start_point: None,
            end_point: None,
            top_plan: None,
            plan_text: None,
            invite_allowed: false,
            invite_text: None,
        }
    }
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

        let cook_and_run_data = Self::load_cook_and_run_data(&storage)
            .map_err(|e| format!("Cook and run data could not be loaded: {}", e))?;

        Ok(LocalStorage {
            storage,
            cook_and_run_data,
        })
    }

    fn load_cook_and_run_data(storage: &web_sys::Storage) -> Result<Vec<CookAndRunData>, String> {
        let data = storage
            .get_item(DATA_KEY)
            .map_err(|e| {
                format!(
                    "Data could not be loaded: {}",
                    e.as_string().unwrap_or_default()
                )
            })?
            .unwrap_or_else(|| "[]".to_string());

        let cook_and_run_data =
            serde_json::from_str(&data).map_err(|e| format!("Data could not parse json: {}", e))?;

        return Ok(cook_and_run_data);
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
}

impl Storage for LocalStorage {
    async fn create_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        cook_and_run: &CookAndRunCreate,
    ) -> Result<(), String> {
        let mut new_data = self.cook_and_run_data.clone();
        for data in &mut new_data {
            if data.id == cook_and_run_id {
                return Err(format!(
                    "Cook and run project with ID {} already exists",
                    cook_and_run_id
                ));
            }
        }

        new_data.push(cook_and_run.to_cook_and_run_data(cook_and_run_id));

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

    async fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String> {
        self.delete_cook_and_run_data(id)
    }

    async fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String> {
        let result = self.get_cook_and_run_data_by_id(id);
        match result {
            Some(data) => Ok(data),
            None => Err(format!("Cook and run project with ID {} not found", id)),
        }
    }
    async fn select_cook_and_run_meta(&self, id: Uuid) -> Result<CookAndRunMetaData, String> {
        let result = self.get_cook_and_run_data_by_id(id);
        match result {
            Some(data) => Ok(data.to_meta()),
            None => Err(format!("Cook and run project with ID {} not found", id)),
        }
    }

    async fn select_cook_and_run_meta_list(&self) -> Result<Vec<CookAndRunMetaData>, String> {
        Ok(self.cook_and_run_data.iter().map(|x| x.to_meta()).collect())
    }

    async fn update_meta_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        cook_and_run_meta: &CookAndRunMetaUpdate,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;
        update_meta(&mut cook_and_run, cook_and_run_id, cook_and_run_meta)?;
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn update_plan_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        plan: &PlanData,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;
        cook_and_run.top_plan = Some(plan.clone());
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn create_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
        course: &CourseCreate,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        if cook_and_run.course_list.iter().any(|c| c.id == course_id) {
            return Err(format!(
                "Course with ID {} already exists in Cook and Run project {}",
                course_id, cook_and_run_id
            ));
        }

        cook_and_run.course_list.push(course.to_local(course_id));
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn update_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
        course: &CourseUpdate,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        let mut found = false;
        for c in &mut cook_and_run.course_list {
            if c.id == course_id {
                *c = course.to_local(course_id);
                found = true;
                break;
            }
        }
        if !found {
            return Err(format!(
                "Course with ID {} not found in Cook and Run project {}",
                course_id, cook_and_run_id
            ));
        }

        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn delete_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        let original_len = cook_and_run.course_list.len();
        cook_and_run.course_list.retain(|c| c.id != course_id);

        if cook_and_run.course_list.len() == original_len {
            return Err(format!(
                "Course with ID {} not found in Cook and Run project {}",
                course_id, cook_and_run_id
            ));
        }

        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn update_course_with_more_hosts_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        if !cook_and_run.course_list.iter().any(|c| c.id == course_id) {
            return Err(format!(
                "Course with ID {} not found in Cook and Run project {}",
                course_id, cook_and_run_id
            ));
        }

        cook_and_run.course_with_more_hosts = Some(course_id);
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn create_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        team: &TeamCreate,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        if cook_and_run.team_list.iter().any(|t| t.id == team_id) {
            return Err(format!(
                "Team with ID {} already exists in Cook and Run project {}",
                team_id, cook_and_run_id
            ));
        }
        cook_and_run.edited = Utc::now().naive_utc();
        cook_and_run.team_list.push(team.to_local(team_id));
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn update_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        team: &TeamUpdate,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        let mut found = false;
        for t in &mut cook_and_run.team_list {
            if t.id == team_id {
                *t = team.to_local(team_id, t.created);
                found = true;
                break;
            }
        }
        if !found {
            return Err(format!(
                "Team with ID {} not found in Cook and Run project {}",
                team_id, cook_and_run_id
            ));
        }
        cook_and_run.edited = Utc::now().naive_utc();
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn delete_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        let original_len = cook_and_run.team_list.len();
        cook_and_run.team_list.retain(|t| t.id != team_id);

        if cook_and_run.team_list.len() == original_len {
            return Err(format!(
                "Team with ID {} not found in Cook and Run project {}",
                team_id, cook_and_run_id
            ));
        }

        cook_and_run.edited = Utc::now().naive_utc();
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn create_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_id: Uuid,
        note_data: &NoteCreate,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        let team = cook_and_run
            .team_list
            .iter_mut()
            .find(|t| t.id == team_id)
            .ok_or_else(|| {
                format!(
                    "Team with ID {} not found in Cook and Run project {}",
                    team_id, cook_and_run_id
                )
            })?;

        if team.note_list.iter().any(|n| n.id == note_id) {
            return Err(format!(
                "Note with ID {} already exists in Team {} of Cook and Run project {}",
                note_id, team_id, cook_and_run_id
            ));
        }

        team.note_list.push(note_data.to_note(note_id));
        cook_and_run.edited = Utc::now().naive_utc();
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn delete_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_id: Uuid,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        let team = cook_and_run
            .team_list
            .iter_mut()
            .find(|t| t.id == team_id)
            .ok_or_else(|| {
                format!(
                    "Team with ID {} not found in Cook and Run project {}",
                    team_id, cook_and_run_id
                )
            })?;

        let original_len = team.note_list.len();
        team.note_list.retain(|n| n.id != note_id);

        if team.note_list.len() == original_len {
            return Err(format!(
                "Note with ID {} not found in Team {} of Cook and Run project {}",
                note_id, team_id, cook_and_run_id
            ));
        }

        cook_and_run.edited = Utc::now().naive_utc();
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn update_start_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        start_point: &Option<MeetingPointData>,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        cook_and_run.start_point = start_point.clone();
        cook_and_run.edited = Utc::now().naive_utc();
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn update_end_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        end_point: &Option<MeetingPointData>,
    ) -> Result<(), String> {
        let mut cook_and_run = self
            .get_cook_and_run_data_by_id(cook_and_run_id)
            .ok_or_else(|| format!("Cook and run project with ID {} not found", cook_and_run_id))?;

        cook_and_run.end_point = end_point.clone();
        cook_and_run.edited = Utc::now().naive_utc();
        self.update_cook_and_run_data(&cook_and_run)
    }

    async fn select_cook_and_run_team_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<TeamData>, String> {
        let cook_and_run = self.select_cook_and_run(cook_and_run_id).await?;
        Ok(cook_and_run.team_list)
    }

    async fn select_cook_and_run_start_point(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<MeetingPointData>, String> {
        let cook_and_run = self.select_cook_and_run(cook_and_run_id).await?;
        Ok(cook_and_run.start_point)
    }

    async fn select_cook_and_run_end_point(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<MeetingPointData>, String> {
        let cook_and_run = self.select_cook_and_run(cook_and_run_id).await?;
        Ok(cook_and_run.end_point)
    }

    async fn select_cook_and_run_course_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<CourseData>, String> {
        let cook_and_run = self.select_cook_and_run(cook_and_run_id).await?;
        Ok(cook_and_run.course_list)
    }
}
fn update_meta(
    cook_and_run: &mut CookAndRunData,
    cook_and_run_id: Uuid,
    cook_and_run_meta: &CookAndRunMetaUpdate,
) -> Result<(), String> {
    if cook_and_run.id != cook_and_run_id {
        return Err(format!(
            "CookAndRunMetaData ID does not match CookAndRunData ID: {} != {}",
            cook_and_run.id, cook_and_run_id
        ));
    }
    cook_and_run.name = cook_and_run_meta.name.clone();
    cook_and_run.occur = cook_and_run_meta.occur.clone();
    cook_and_run.edited = Utc::now().naive_utc();
    Ok(())
}
