use chrono::NaiveDateTime;
use dioxus::{
    hooks::use_context,
    signals::{Readable, Signal},
};
use serde::Deserialize;
use uuid::Uuid;
use web_sys::console;

use crate::{
    auth0::AuthState,
    storage::{CookAndRunData, CookAndRunMetaData, MeetingPointData, Storage},
};

#[derive(PartialEq, Clone, Debug)]
pub struct CloudStorage {
    base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CookAndRunMetaResponse {
    pub data: Vec<CookAndRunMetaDataResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CookAndRunMetaDataResponse {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
}

impl CookAndRunMetaDataResponse {
    pub fn to_cook_and_run_meta_data(&self) -> CookAndRunMetaData {
        CookAndRunMetaData {
            id: self.id,
            name: self.name.clone(),
            created: self.created,
            edited: self.edited,
            occur: self.occur,
            is_in_cloud: true,
        }
    }
}

impl CloudStorage {
    pub fn new() -> Self {
        CloudStorage {
            base_url: "http://0.0.0.0:3000".to_string(),
        }
    }
}

fn get_access_token() -> String {
    "dummy_access_token".to_string()
}

impl Storage for CloudStorage {
    async fn create_cook_and_run(&mut self, cook_and_run: &CookAndRunData) -> Result<(), String> {
        let url = format!("{}/cook_and_run/{}", self.base_url, cook_and_run.id);
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(get_access_token())
            .json(&cook_and_run)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String> {
        let url = format!("{}/cook_and_run/{}", self.base_url, id);
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(get_access_token())
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String> {
        let url = format!("{}/cook_and_run/{}", self.base_url, id);
        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(get_access_token())
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => response
                .json::<CookAndRunData>()
                .await
                .map_err(|e| e.to_string()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run_meta_list(
        &self,
    ) -> Result<Vec<super::CookAndRunMetaData>, String> {
        let auth_state = use_context::<Signal<AuthState>>();
        let auth_state = auth_state.read();

        let session_data = match auth_state.clone() {
            AuthState::LoggedIn(session_data) => session_data,
            _ => {
                return Ok(vec![]);
            }
        };

        let url = format!(
            "{}/cook_and_run?userId={}&minimal=true",
            self.base_url,
            urlencoding::encode(&session_data.user.sub)
        );

        console::log_1(&format!("Fetching CookAndRunMetaData from URL: {}", url).into());
        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(&session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => {
                response.json::<CookAndRunMetaResponse>().await.map_or_else(
                    |e| Err(e.to_string()),
                    |data| {
                        Ok(data
                            .data
                            .iter()
                            .map(|c_a_r_mini| c_a_r_mini.to_cook_and_run_meta_data())
                            .collect())
                    },
                )
            }

            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_meta_of_cook_and_run(
        &mut self,
        cook_and_run_meta: &CookAndRunMetaData,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/meta",
            self.base_url, cook_and_run_meta.id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(get_access_token())
            .json(cook_and_run_meta)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_plan_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        plan: &super::PlanData,
    ) -> Result<(), String> {
        let url = format!("{}/cook_and_run/{}/plan", self.base_url, cook_and_run_id);
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(get_access_token())
            .json(plan)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }
    async fn create_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course: &super::CourseData,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/course/{}",
            self.base_url, cook_and_run_id, course.id
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(get_access_token())
            .json(course)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course: &super::CourseData,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/course/{}",
            self.base_url, cook_and_run_id, course.id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(get_access_token())
            .json(course)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn delete_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/course/{}",
            self.base_url, cook_and_run_id, course_id
        );
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(get_access_token())
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_course_with_more_hosts_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/course/{}/with_more_hosts",
            self.base_url, cook_and_run_id, course_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(get_access_token())
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn create_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team: &super::TeamData,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/team/{}",
            self.base_url, cook_and_run_id, team.id
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(get_access_token())
            .json(team)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team: &super::TeamData,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/team/{}",
            self.base_url, cook_and_run_id, team.id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(get_access_token())
            .json(team)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn delete_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/team/{}",
            self.base_url, cook_and_run_id, team_id
        );
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(get_access_token())
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn create_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_data: &super::NoteData,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/team/{}/note/{}",
            self.base_url, cook_and_run_id, team_id, note_data.id
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(get_access_token())
            .json(note_data)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn delete_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_id: Uuid,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/team/{}/note/{}",
            self.base_url, cook_and_run_id, team_id, note_id
        );
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(get_access_token())
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_start_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        start_point: &Option<MeetingPointData>,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/start_point",
            self.base_url, cook_and_run_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(get_access_token())
            .json(start_point)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_end_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        end_point: &Option<MeetingPointData>,
    ) -> Result<(), String> {
        let url = format!(
            "{}/cook_and_run/{}/end_point",
            self.base_url, cook_and_run_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(get_access_token())
            .json(end_point)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }
}
