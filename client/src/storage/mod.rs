mod cloud;
mod local;
pub mod mapper;

use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use web_sys::console;

use std::f64::consts::PI;

use crate::{
    keycloak::AuthState, storage::{cloud::CloudStorage, local::LocalStorage},
};

#[derive(Debug, Clone)]
pub struct StorageManager {
    local: LocalStorage,
    cloud: Option<CloudStorage>,
    backend_available: bool,
}

async fn transfere<T: Storage>(storage: &mut T, c_a_r: CookAndRunData) -> Result<Uuid, String> {
    let cook_and_run_id = Uuid::new_v4();
    let c_a_r_create = CookAndRunCreate { name: c_a_r.name };
    let result = storage
        .create_cook_and_run(cook_and_run_id, &c_a_r_create)
        .await;
    if let Err(e) = result {
        return Err(format!(
            "Error while loading cook and run from storage: {}",
            e
        ));
    };

    for team in c_a_r.team_list {
        let team_id = Uuid::new_v4();

        let team_create = TeamCreate {
            name: team.name,
            address: team.address,
            mail: team.mail,
            phone: team.phone,
            members: team.members,
            diets: team.diets,
            needs_check: team.needs_check,
        };

        let result = storage
            .create_team_of_cook_and_run(cook_and_run_id, team_id, &team_create)
            .await;
        if let Err(e) = result {
            return Err(format!(
                "Error while loading cook and run from storage: {}",
                e
            ));
        };
    }
    Ok(cook_and_run_id)
}

impl StorageManager {
    pub fn new() -> Result<Self, String> {
        Ok(StorageManager {
            local: LocalStorage::new()?,
            cloud: None,
            backend_available: false,
        })
    }

    pub async fn load_cloud(
        &mut self,
        auth_state: AuthState,
        base_url: String,
    ) -> Result<(), String> {
        let cloud = CloudStorage::new(auth_state, base_url).await;
        match cloud {
            Ok(cloud) => {
                self.cloud = Some(cloud);
                self.backend_available = true;
                Ok(())
            }
            Err(e) => {
                self.cloud = None;
                self.backend_available = false;
                Err(format!("Error while loading cloud connection: {}", e))
            }
        }
    }

    pub fn disconnect_cloud(&mut self) {
        self.cloud = None;
    }

    pub fn is_backend_available(&self) -> bool {
        self.backend_available
    }

    pub fn is_cloud_available(&self) -> bool {
        self.cloud.is_some()
    }

    fn get_cloud_mut(&mut self) -> Result<&mut CloudStorage, String> {
        if let Some(cloud) = self.cloud.as_mut() {
            Ok(cloud)
        } else {
            Err("Cloud is not configured".to_string())
        }
    }

    fn get_cloud(&self) -> Result<&CloudStorage, String> {
        if let Some(cloud) = self.cloud.as_ref() {
            Ok(cloud)
        } else {
            Err("Cloud is not configured".to_string())
        }
    }

    pub async fn upload_to_cloud(&mut self, cook_and_run_id: Uuid) -> Result<Uuid, String> {
        let local = &mut self.local.clone();
        let cloud = self.get_cloud_mut()?;
        let result = local.select_cook_and_run(cook_and_run_id).await;
        let result = match result {
            Ok(c_a_r) => c_a_r,
            Err(e) => {
                return Err(format!(
                    "Error while loading cook and run from cloud: {}",
                    e
                ))
            }
        };
        let new_cook_and_run_id = transfere(cloud, result).await?;
        local.delete_cook_and_run(cook_and_run_id).await?;
        Ok(new_cook_and_run_id)
    }

    pub async fn download_from_cloud(&mut self, cook_and_run_id: Uuid) -> Result<Uuid, String> {
        let local = &mut self.local.clone();
        let cloud = self.get_cloud_mut()?;
        let result = cloud.select_cook_and_run(cook_and_run_id).await;
        let result = match result {
            Ok(c_a_r) => c_a_r,
            Err(e) => {
                return Err(format!(
                    "Error while loading cook and run from cloud: {}",
                    e
                ))
            }
        };
        let new_cook_and_run_id = transfere(local, result).await?;
        cloud.delete_cook_and_run(cook_and_run_id).await?;
        Ok(new_cook_and_run_id)
    }

    pub async fn create_from_file(&mut self, cook_and_run: CookAndRunData) -> Result<Uuid, String> {
        transfere(&mut self.local, cook_and_run).await
    }

    pub async fn create_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        cook_and_run: &CookAndRunCreate,
    ) -> Result<(), String> {
        self.local
            .create_cook_and_run(cook_and_run_id, cook_and_run)
            .await
    }

    pub async fn delete_cook_and_run(&mut self, cook_and_run_id: Uuid) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local.delete_cook_and_run(cook_and_run_id).await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud.delete_cook_and_run(cook_and_run_id).await
        }
    }

    pub async fn update_meta_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        cook_and_run_meta: &CookAndRunMetaUpdate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_meta_of_cook_and_run(cook_and_run_id, cook_and_run_meta)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_meta_of_cook_and_run(cook_and_run_id, cook_and_run_meta)
                .await
        }
    }

    pub async fn update_plan_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        plan: &PlanData,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_plan_of_cook_and_run(cook_and_run_id, plan)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_plan_of_cook_and_run(cook_and_run_id, plan)
                .await
        }
    }

    pub async fn select_plan_of_cook_and_run(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<PlanData>, String> {
        match self
            .local
            .select_plan_of_cook_and_run(cook_and_run_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(e_local) => match self
                .get_cloud()?
                .select_plan_of_cook_and_run(cook_and_run_id)
                .await
            {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    cook_and_run_id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn delete_plan_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .delete_plan_of_cook_and_run(cook_and_run_id)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud.delete_plan_of_cook_and_run(cook_and_run_id).await
        }
    }

    pub async fn update_plan_config_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        plan_config: &PlanConfigData,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_plan_config_of_cook_and_run(cook_and_run_id, plan_config)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_plan_config_of_cook_and_run(cook_and_run_id, plan_config)
                .await
        }
    }

    pub async fn select_plan_config_of_cook_and_run(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<PlanConfigData>, String> {
        match self
            .local
            .select_plan_config_of_cook_and_run(cook_and_run_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(e_local) => match self
                .get_cloud()?
                .select_plan_config_of_cook_and_run(cook_and_run_id)
                .await
            {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    cook_and_run_id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn delete_plan_config_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .delete_plan_config_of_cook_and_run(cook_and_run_id)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .delete_plan_config_of_cook_and_run(cook_and_run_id)
                .await
        }
    }

    pub async fn create_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
        course: &CourseCreate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .create_course_of_cook_and_run(cook_and_run_id, course_id, course)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .create_course_of_cook_and_run(cook_and_run_id, course_id, course)
                .await
        }
    }

    pub async fn update_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
        course: &CourseUpdate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_course_of_cook_and_run(cook_and_run_id, course_id, course)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_course_of_cook_and_run(cook_and_run_id, course_id, course)
                .await
        }
    }

    pub async fn delete_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .delete_course_of_cook_and_run(cook_and_run_id, course_id)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .delete_course_of_cook_and_run(cook_and_run_id, course_id)
                .await
        }
    }

    pub async fn update_course_with_more_hosts_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_course_with_more_hosts_of_cook_and_run(cook_and_run_id, course_id)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_course_with_more_hosts_of_cook_and_run(cook_and_run_id, course_id)
                .await
        }
    }

    pub async fn create_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        team: &TeamCreate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .create_team_of_cook_and_run(cook_and_run_id, team_id, team)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .create_team_of_cook_and_run(cook_and_run_id, team_id, team)
                .await
        }
    }

    pub async fn update_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        team: &TeamUpdate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_team_of_cook_and_run(cook_and_run_id, team_id, team)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_team_of_cook_and_run(cook_and_run_id, team_id, team)
                .await
        }
    }

    pub async fn delete_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .delete_team_of_cook_and_run(cook_and_run_id, team_id)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .delete_team_of_cook_and_run(cook_and_run_id, team_id)
                .await
        }
    }

    pub async fn select_cook_and_run_team_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<TeamData>, String> {
        match self
            .local
            .select_cook_and_run_team_list(cook_and_run_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(e_local) => match self
                .get_cloud()?
                .select_cook_and_run_team_list(cook_and_run_id)
                .await
            {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    cook_and_run_id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn select_cook_and_run_team(
        &self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
    ) -> Result<TeamData, String> {
        match self
            .local
            .select_cook_and_run_team(cook_and_run_id, team_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(e_local) => match self
                .get_cloud()?
                .select_cook_and_run_team(cook_and_run_id, team_id)
                .await
            {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    cook_and_run_id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn create_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_id: Uuid,
        note_data: &NoteCreate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .create_team_note_of_cook_and_run(cook_and_run_id, team_id, note_id, note_data)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .create_team_note_of_cook_and_run(cook_and_run_id, team_id, note_id, note_data)
                .await
        }
    }

    pub async fn delete_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_id: Uuid,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .delete_team_note_of_cook_and_run(cook_and_run_id, team_id, note_id)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .delete_team_note_of_cook_and_run(cook_and_run_id, team_id, note_id)
                .await
        }
    }

    pub async fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String> {
        match self.local.select_cook_and_run(id).await {
            Ok(data) => Ok(data),
            Err(e_local) => match self.get_cloud()?.select_cook_and_run(id).await {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn select_cook_and_run_meta(&self, id: Uuid) -> Result<CookAndRunMetaData, String> {
        match self.local.select_cook_and_run_meta(id).await {
            Ok(data) => Ok(data),
            Err(e_local) => match self.get_cloud()?.select_cook_and_run_meta(id).await {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn select_cook_and_run_meta_list(&self) -> Result<Vec<CookAndRunMetaData>, String> {
        let local_data = self.local.select_cook_and_run_meta_list().await?;

        let cloud: &CloudStorage = match self.get_cloud() {
            Ok(cloud) => cloud,
            Err(e) => {
                console::warn_1(&format!("Error when loading cloud connection: {}", e).into());
                return Ok(local_data);
            }
        };

        let cloud_data = match cloud.select_cook_and_run_meta_list().await {
            Ok(data) => data,
            Err(e) => {
                console::warn_1(
                    &format!(
                        "Error when loading all cook and run projects from cloud: {}",
                        e
                    )
                    .into(),
                );

                Vec::new()
            }
        };

        let mut combined_data = HashMap::new();
        for data in cloud_data {
            combined_data.insert(data.id, data);
        }
        for data in local_data {
            combined_data.entry(data.id).or_insert(data);
        }

        Ok(combined_data.into_values().collect())
    }

    pub async fn update_start_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        start_point: &Option<MeetingPointData>,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_start_point_in_cook_and_run(cook_and_run_id, start_point)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_start_point_in_cook_and_run(cook_and_run_id, start_point)
                .await
        }
    }

    pub async fn update_end_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        end_point: &Option<MeetingPointData>,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_end_point_in_cook_and_run(cook_and_run_id, end_point)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_end_point_in_cook_and_run(cook_and_run_id, end_point)
                .await
        }
    }

    pub async fn create_cook_and_run_share_config(
        &mut self,
        cook_and_run_id: Uuid,
        share_config: &ShareTeamConfigCreate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .create_cook_and_run_share_config(cook_and_run_id, share_config)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .create_cook_and_run_share_config(cook_and_run_id, share_config)
                .await
        }
    }

    pub async fn update_cook_and_run_share_config(
        &mut self,
        cook_and_run_id: Uuid,
        share_config: &ShareTeamConfigCreate,
    ) -> Result<(), String> {
        let exists_local = self
            .local
            .select_cook_and_run(cook_and_run_id)
            .await
            .is_ok();

        if exists_local {
            self.local
                .update_cook_and_run_share_config(cook_and_run_id, share_config)
                .await
        } else {
            let cloud = self.get_cloud_mut()?;
            cloud
                .update_cook_and_run_share_config(cook_and_run_id, share_config)
                .await
        }
    }

    pub async fn select_cook_and_run_start_point(
        &self,
        id: Uuid,
    ) -> Result<Option<MeetingPointData>, String> {
        match self.local.select_cook_and_run_start_point(id).await {
            Ok(data) => Ok(data),
            Err(e_local) => match self.get_cloud()?.select_cook_and_run_start_point(id).await {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn select_cook_and_run_end_point(
        &self,
        id: Uuid,
    ) -> Result<Option<MeetingPointData>, String> {
        match self.local.select_cook_and_run_end_point(id).await {
            Ok(data) => Ok(data),
            Err(e_local) => match self.get_cloud()?.select_cook_and_run_end_point(id).await {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn select_cook_and_run_course_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<CourseData>, String> {
        match self
            .local
            .select_cook_and_run_course_list(cook_and_run_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(e_local) => match self
                .get_cloud()?
                .select_cook_and_run_course_list(cook_and_run_id)
                .await
            {
                Ok(data) => Ok(data),
                Err(e_cloud) => Err(format!(
                    "Cook and run with id {} not found in local or cloud storage: {} | {}",
                    cook_and_run_id, e_local, e_cloud
                )),
            },
        }
    }

    pub async fn select_cook_and_run_share_config(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<ShareTeamConfig>, String> {
        self.get_cloud()?
            .select_cook_and_run_share_config(cook_and_run_id)
            .await
    }
}

pub trait Storage {
    async fn create_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        cook_and_rund_data: &CookAndRunCreate,
    ) -> Result<(), String>;
    async fn delete_cook_and_run(&mut self, id: Uuid) -> Result<(), String>;
    async fn update_meta_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        cook_and_run_meta: &CookAndRunMetaUpdate,
    ) -> Result<(), String>;
    async fn select_plan_of_cook_and_run(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<PlanData>, String>;
    async fn update_plan_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        plan: &PlanData,
    ) -> Result<(), String>;
    async fn delete_plan_of_cook_and_run(&mut self, cook_and_run_id: Uuid) -> Result<(), String>;
    async fn select_plan_config_of_cook_and_run(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<PlanConfigData>, String>;
    async fn update_plan_config_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        plan_config: &PlanConfigData,
    ) -> Result<(), String>;
    async fn delete_plan_config_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
    ) -> Result<(), String>;
    async fn create_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
        course: &CourseCreate,
    ) -> Result<(), String>;
    async fn update_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
        course: &CourseUpdate,
    ) -> Result<(), String>;
    async fn delete_course_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String>;
    async fn update_course_with_more_hosts_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), String>;
    async fn create_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        team: &TeamCreate,
    ) -> Result<(), String>;
    async fn update_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        team: &TeamUpdate,
    ) -> Result<(), String>;
    async fn delete_team_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), String>;

    async fn create_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_id: Uuid,
        note_data: &NoteCreate,
    ) -> Result<(), String>;

    async fn delete_team_note_of_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
        note_id: Uuid,
    ) -> Result<(), String>;

    async fn update_start_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        start_point: &Option<MeetingPointData>,
    ) -> Result<(), String>;

    async fn update_end_point_in_cook_and_run(
        &mut self,
        cook_and_run_id: Uuid,
        end_point: &Option<MeetingPointData>,
    ) -> Result<(), String>;

    async fn create_cook_and_run_share_config(
        &mut self,
        cook_and_run_id: Uuid,
        share_config: &ShareTeamConfigCreate,
    ) -> Result<(), String>;

    async fn update_cook_and_run_share_config(
        &mut self,
        cook_and_run_id: Uuid,
        share_config: &ShareTeamConfigCreate,
    ) -> Result<(), String>;

    async fn select_cook_and_run_meta_list(&self) -> Result<Vec<CookAndRunMetaData>, String>;
    async fn select_cook_and_run(&self, id: Uuid) -> Result<CookAndRunData, String>;
    async fn select_cook_and_run_meta(&self, id: Uuid) -> Result<CookAndRunMetaData, String>;
    async fn select_cook_and_run_team_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<TeamData>, String>;
    async fn select_cook_and_run_team(
        &self,
        cook_and_run_id: Uuid,
        team_id: Uuid,
    ) -> Result<TeamData, String>;
    async fn select_cook_and_run_start_point(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<MeetingPointData>, String>;
    async fn select_cook_and_run_end_point(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Option<MeetingPointData>, String>;
    async fn select_cook_and_run_course_list(
        &self,
        cook_and_run_id: Uuid,
    ) -> Result<Vec<CourseData>, String>;
    async fn select_cook_and_run_share_config(
        &self,
        id: Uuid,
    ) -> Result<Option<ShareTeamConfig>, String>;
}

#[derive(Debug, Serialize)]
pub struct CourseCreate {
    pub name: String,
    pub time: NaiveTime,
    pub has_multiple_hosts: bool,
}

#[derive(Debug, Serialize)]
pub struct CourseUpdate {
    pub name: String,
    pub time: NaiveTime,
    pub has_multiple_hosts: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseData {
    pub id: Uuid,
    pub name: String,
    pub time: NaiveTime,
    pub has_multiple_hosts: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostingData {
    pub id: Uuid,
    /*Course ID*/
    pub name: Uuid,
    /*Team ID */
    pub host: Uuid,
    /*Team ID */
    pub guest_list: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct TeamData {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub address: AddressData,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u8>,
    pub diets: Option<String>,
    pub needs_check: bool,
    pub note_list: Vec<NoteData>,
}

impl Default for TeamData {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "[Team Name]".to_string(),
            created: Utc::now(),
            edited: Utc::now(),
            address: AddressData::default(),
            mail: Some("[mail address]".to_string()),
            phone: Some("[phone number]".to_string()),
            members: Some(2),
            diets: Some("[some diets]".to_string()),
            needs_check: false,
            note_list: vec![],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TeamCreate {
    pub name: String,
    pub address: AddressData,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u8>,
    pub diets: Option<String>,
    pub needs_check: bool,
}

#[derive(Debug, Serialize)]
pub struct TeamUpdate {
    pub name: String,
    pub address: AddressData,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u8>,
    pub diets: Option<String>,
    pub needs_check: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressData {
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl PartialEq for AddressData {
    fn eq(&self, other: &Self) -> bool {
        self.address.eq(&other.address)
    }
}

impl Eq for AddressData {}

impl Hash for AddressData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl ToString for AddressData {
    fn to_string(&self) -> String {
        format!(
            "{}",
            self.address
        )
    }
}

impl AddressData {
    fn deg_to_rad(deg: f64) -> f64 {
        deg * PI / 180.0
    }

    pub fn distance(&self, addr: &AddressData) -> f64 {
        let r = 6371.0;

        let dlat = Self::deg_to_rad(addr.latitude - self.latitude);
        let dlon = Self::deg_to_rad(addr.longitude - self.longitude);

        let lat1_rad = Self::deg_to_rad(self.latitude);
        let lat2_rad = Self::deg_to_rad(addr.latitude);

        let a = (dlat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().asin();

        r * c
    }
}

impl Default for AddressData {
    fn default() -> Self {
        Self {
            address: "[Address]".to_string(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NoteCreate {
    pub headline: String,
    pub content: String,
}

impl NoteCreate {
    pub fn to_note(&self, note_id: Uuid) -> NoteData {
        NoteData {
            id: note_id,
            headline: self.headline.clone(),
            content: self.content.clone(),
            created: Utc::now(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct NoteData {
    pub id: Uuid,
    pub headline: String,
    pub content: String,
    pub created: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingPointData {
    pub name: String,
    pub time: NaiveTime,
    pub address: AddressData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanData {
    pub hosting_list: Vec<HostingData>,
    pub walking_path: HashMap<Uuid /*Team ID */, Vec<Uuid /*Hosting ID */>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanConfigData {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub language: Language,
}

impl Default for PlanConfigData {
    fn default() -> Self {
        Self {
            title: "Plan".to_string(),
            description: "<center>This is the plan for the cook and run event.</center>"
                .to_string(),
            date: Utc::now().naive_utc().date(),
            language: Language::English,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    English,
    German,
}

impl Language {
    pub fn from_string(value: String) -> Self {
        match value.as_str() {
            "deu" => Language::German,
            _ => Language::English,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Language::German => "deu".to_string(),
            Language::English => "eng".to_string(),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let iso = String::deserialize(deserializer)?;
        Self::from_iso639_3(&iso)
            .ok_or_else(|| serde::de::Error::custom(format!("Unknown language code: {}", iso)))
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let iso = self.to_iso639_3();
        serializer.serialize_str(iso)
    }
}

impl Language {
    pub fn to_iso639_3(&self) -> &str {
        match self {
            Language::English => "eng",
            Language::German => "deu",
        }
    }

    pub fn from_iso639_3(iso: &str) -> Option<Self> {
        match iso {
            "eng" => Some(Language::English),
            "deu" => Some(Language::German),
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookAndRunData {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub occur: DateTime<Utc>,
    pub is_in_cloud: bool,
    pub team_list: Vec<TeamData>,
    pub course_list: Vec<CourseData>,
    pub course_with_more_hosts: Option<Uuid>,
    pub start_point: Option<MeetingPointData>,
    pub end_point: Option<MeetingPointData>,
    pub top_plan: Option<PlanData>,
    pub plan_config: Option<PlanConfigData>,
    pub plan_text: Option<String>,
    pub invite_allowed: bool,
    pub invite_text: Option<String>,
}

impl CookAndRunData {
    pub fn new(id: Uuid, name: String) -> Self {
        CookAndRunData {
            id,
            name,
            created: Utc::now(),
            edited: Utc::now(),
            occur: Utc::now(),
            is_in_cloud: false,
            team_list: vec![],
            course_list: vec![],
            course_with_more_hosts: None,
            start_point: None,
            end_point: None,
            top_plan: None,
            plan_config: None,
            plan_text: None,
            invite_allowed: false,
            invite_text: None,
        }
    }

    pub fn to_meta(&self) -> CookAndRunMetaData {
        CookAndRunMetaData {
            id: self.id,
            name: self.name.clone(),
            created: self.created,
            edited: self.edited,
            occur: self.occur,
            is_in_cloud: self.is_in_cloud,
        }
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("Failed to serialize CookAndRunData: {}", e))
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("Failed to deserialize CookAndRunData: {}", e))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookAndRunMetaData {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub occur: DateTime<Utc>,
    #[serde(default)]
    pub is_in_cloud: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CookAndRunMetaUpdate {
    pub name: String,
    pub occur: DateTime<Utc>,
}

impl CookAndRunMetaData {
    pub fn new(id: Uuid, name: String, occur: DateTime<Utc>) -> Self {
        CookAndRunMetaData {
            id,
            name,
            created: Utc::now(),
            edited: Utc::now(),
            occur,
            is_in_cloud: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CookAndRunCreate {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RequiredField {
    Mail,
    Phone,
    Members,
    Diets,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShareTeamConfig {
    pub invite_text: String,
    pub needs_login: bool,
    pub default_needs_check: bool,
    pub required_fields: Vec<RequiredField>,
    pub max_teams: Option<u32>,
    pub registration_deadline: Option<DateTime<chrono::Utc>>,
    pub created: DateTime<chrono::Utc>,
}

impl ShareTeamConfig {
    pub fn to_create(&self) -> ShareTeamConfigCreate {
        ShareTeamConfigCreate {
            invite_text: self.invite_text.clone(),
            needs_login: self.needs_login,
            default_needs_check: self.default_needs_check,
            required_fields: self.required_fields.clone(),
            max_teams: self.max_teams,
            registration_deadline: self.registration_deadline,
        }
    }
}

impl Default for ShareTeamConfig {
    fn default() -> Self {
        Self {
            invite_text: "".to_string(),
            needs_login: false,
            default_needs_check: true,
            required_fields: vec![RequiredField::Mail, RequiredField::Diets],
            max_teams: None,
            registration_deadline :Some(get_future_utc_at_local_1800()),
            created: Utc::now(),
        }
    }
 
}

fn get_future_utc_at_local_1800() -> DateTime<Utc> {
    let now_local = Local::now();

    let target_date = now_local.date_naive() + Duration::days(28);

    let target_time = NaiveTime::from_hms_opt(18, 0, 0)
        .expect("Ungültige Zeitangabe");
    let target_naive = target_date.and_time(target_time);

    let target_local = Local
        .from_local_datetime(&target_naive)
        .single()
        .expect("Uhrzeit ist in der lokalen Zeitzone ungültig oder mehrdeutig");

    target_local.with_timezone(&Utc)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShareTeamConfigCreate {
    pub invite_text: String,
    pub needs_login: bool,
    pub default_needs_check: bool,
    pub required_fields: Vec<RequiredField>,
    pub max_teams: Option<u32>,
    pub registration_deadline: Option<DateTime<chrono::Utc>>,
}
