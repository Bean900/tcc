use chrono::NaiveDateTime;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::console;

use crate::{
    auth0::{AuthState, SessionData},
    storage::{
        AddressData, CookAndRunCreate, CookAndRunData, CookAndRunMetaData, CookAndRunMetaUpdate,
        CourseCreate, CourseData, CourseUpdate, MeetingPointData, NoteCreate, Storage, TeamCreate,
        TeamData, TeamUpdate,
    },
};

#[derive(Clone, Debug)]
pub struct CloudStorage {
    base_url: String,
    auth_state: AuthState,
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

#[derive(Debug, Clone, Serialize)]
struct CookAndRunCreateRequest {
    name: String,
    #[serde(rename = "userId")]
    user_id: String,
}

impl CookAndRunCreateRequest {
    fn from(c_a_r: &CookAndRunCreate, user_id: String) -> Self {
        CookAndRunCreateRequest {
            name: c_a_r.name.clone(),
            user_id: user_id,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TeamListResponse {
    data: Vec<TeamData>,
}

#[derive(Debug, Serialize)]
pub struct TeamCreateRequest {
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub address: AddressData,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u32>,
    pub diets: Option<String>,
    pub needs_check: bool,
}

impl TeamCreateRequest {
    fn from(from: &TeamCreate, user_id: Option<String>) -> Self {
        TeamCreateRequest {
            name: from.name.clone(),
            user_id,
            address: from.address.clone(),
            mail: from.mail.clone(),
            phone: from.phone.clone(),
            members: from.members,
            diets: from.diets.clone(),
            needs_check: from.needs_check,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CourseListResponse {
    data: Vec<CourseData>,
}

impl CloudStorage {
    pub fn new(auth_state: AuthState) -> Self {
        CloudStorage {
            base_url: "http://127.0.0.1:3000".to_string(),
            auth_state,
        }
    }

    fn get_access_token(&self) -> Result<SessionData, String> {
        match self.auth_state.clone() {
            AuthState::LoggedIn(session_data) => Ok(session_data),
            _ => Err("Could not clone auth state!".to_string()),
        }
    }

    pub fn get_auth_state(&self) -> AuthState {
        self.auth_state.clone()
    }

    pub fn set_auth_state(&mut self, auth_state: AuthState) {
        self.auth_state = auth_state;
    }
}

impl Storage for CloudStorage {
    async fn create_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        cook_and_run: &CookAndRunCreate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}", self.base_url, cook_and_run_id);
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(session_data.access_token)
            .json(&CookAndRunCreateRequest::from(
                cook_and_run,
                session_data.user.sub,
            ))
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}", self.base_url, id);
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}", self.base_url, id);
        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(session_data.access_token)
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

    async fn select_cook_and_run_meta(&self, id: Uuid) -> Result<CookAndRunMetaData, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}/metadata", self.base_url, id);
        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(&session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => response
                .json::<CookAndRunMetaDataResponse>()
                .await
                .map_err(|e| e.to_string())
                .map(|data| data.to_cook_and_run_meta_data()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run_meta_list(
        &self,
    ) -> Result<Vec<super::CookAndRunMetaData>, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
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
        cook_and_run_id: Uuid,
        cook_and_run_meta: &CookAndRunMetaUpdate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!(
            "{}/cook_and_run/{}/metadata",
            self.base_url, cook_and_run_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(session_data.access_token)
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
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}/plan", self.base_url, cook_and_run_id);
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(session_data.access_token)
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
        course_id: Uuid,
        course: &CourseCreate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/course/{}",
            self.base_url, cook_and_run_id, course_id
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(session_data.access_token)
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
        course_id: Uuid,
        course: &CourseUpdate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/course/{}",
            self.base_url, cook_and_run_id, course_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(session_data.access_token)
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
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/course/{}",
            self.base_url, cook_and_run_id, course_id
        );
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(session_data.access_token)
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
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/course/{}/with_more_hosts",
            self.base_url, cook_and_run_id, course_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(session_data.access_token)
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
        team_id: Uuid,
        team: &TeamCreate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let request_data = TeamCreateRequest::from(team, Some(session_data.user.sub));

        let url = format!(
            "{}/cook_and_run/{}/team/{}",
            self.base_url, cook_and_run_id, team_id
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(session_data.access_token)
            .json(&request_data)
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
        team_id: Uuid,
        team: &TeamUpdate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/team/{}",
            self.base_url, cook_and_run_id, team_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(session_data.access_token)
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
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/team/{}",
            self.base_url, cook_and_run_id, team_id
        );
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(session_data.access_token)
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
        note_id: Uuid,
        note_data: &NoteCreate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/team/{}/note/{}",
            self.base_url, cook_and_run_id, team_id, note_id
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(session_data.access_token)
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
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/team/{}/note/{}",
            self.base_url, cook_and_run_id, team_id, note_id
        );
        let client = reqwest::Client::new();
        let res = client
            .delete(&url)
            .bearer_auth(session_data.access_token)
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
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/start_point",
            self.base_url, cook_and_run_id
        );
        let client = reqwest::Client::new();
        let res = if let Some(sp) = start_point {
            client
                .patch(&url)
                .bearer_auth(session_data.access_token)
                .json(sp)
                .send()
                .await
        } else {
            client
                .delete(&url)
                .bearer_auth(session_data.access_token)
                .send()
                .await
        };
        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) if response.status() == StatusCode::NOT_FOUND => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_end_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        end_point: &Option<MeetingPointData>,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/end_point",
            self.base_url, cook_and_run_id
        );
        let client = reqwest::Client::new();
        let res = if let Some(ep) = end_point {
            client
                .patch(&url)
                .bearer_auth(session_data.access_token)
                .json(ep)
                .send()
                .await
        } else {
            let res = client
                .delete(&url)
                .bearer_auth(session_data.access_token)
                .send()
                .await;
            res
        };

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) if response.status() == StatusCode::NOT_FOUND => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run_team_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<super::TeamData>, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}/teams", self.base_url, cook_and_run_id);

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(&session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => response
                .json::<TeamListResponse>()
                .await
                .map_or_else(|e| Err(e.to_string()), |data| Ok(data.data)),

            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run_start_point(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<MeetingPointData>, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!(
            "{}/cook_and_run/{}/start_point",
            self.base_url, cook_and_run_id
        );

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(&session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status() == StatusCode::OK => response
                .json::<MeetingPointData>()
                .await
                .map_or_else(|e| Err(e.to_string()), |data| Ok(Some(data))),
            Ok(response) if response.status() == StatusCode::NO_CONTENT => Ok(None),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run_end_point(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<MeetingPointData>, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!(
            "{}/cook_and_run/{}/end_point",
            self.base_url, cook_and_run_id
        );

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(&session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status() == StatusCode::OK => response
                .json::<MeetingPointData>()
                .await
                .map_or_else(|e| Err(e.to_string()), |data| Ok(Some(data))),
            Ok(response) if response.status() == StatusCode::NO_CONTENT => Ok(None),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run_course_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<CourseData>, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}/courses", self.base_url, cook_and_run_id);

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(&session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => response
                .json::<CourseListResponse>()
                .await
                .map_or_else(|e| Err(e.to_string()), |data| Ok(data.data)),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn select_cook_and_run_share_config(
        &self,
        id: Uuid,
    ) -> Result<Option<super::ShareTeamConfig>, String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!("{}/cook_and_run/{}/share_team_config", self.base_url, id);
        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(session_data.access_token)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => response
                .json::<super::ShareTeamConfig>()
                .await
                .map_err(|e| e.to_string())
                .map(Some),
            Ok(response) if response.status() == StatusCode::NOT_FOUND => Ok(None),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn create_cook_and_run_share_config(
        &mut self,
        cook_and_run_id: Uuid,
        share_config: &super::ShareTeamConfigCreate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };

        let url = format!(
            "{}/cook_and_run/{}/share_team_config",
            self.base_url, cook_and_run_id
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .bearer_auth(session_data.access_token)
            .json(share_config)
            .send()
            .await;

        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }

    async fn update_cook_and_run_share_config(
        &mut self,
        cook_and_run_id: Uuid,
        share_config: &super::ShareTeamConfigCreate,
    ) -> Result<(), String> {
        let session_data = match self.get_access_token() {
            Ok(sd) => sd,
            Err(_) => return Err("No auth data!".to_string()),
        };
        let url = format!(
            "{}/cook_and_run/{}/share_team_config",
            self.base_url, cook_and_run_id
        );
        let client = reqwest::Client::new();
        let res = client
            .patch(&url)
            .bearer_auth(session_data.access_token)
            .json(share_config)
            .send()
            .await;
        match res {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Request failed: {}", response.status())),
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }
}
