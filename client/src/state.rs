use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use web_sys::window;

const STORAGE_KEY: &str = "cook_run_consent";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentState {
    pub consent_given: bool,
    pub timestamp: NaiveDateTime,
    pub version: String,
    pub analytics_allowed: bool,
}

impl Default for ConsentState {
    fn default() -> Self {
        Self {
            consent_given: false,
            timestamp: NaiveDateTime::default(),
            version: "1.0".to_string(),
            analytics_allowed: false,
        }
    }
}

impl ConsentState {
    pub fn load(config_version: &str) -> Self {
        match Self::from_storage() {
            Some(stored) if stored.version == config_version => stored,
            _ => Self {
                version: config_version.to_string(),
                ..Default::default()
            },
        }
    }

    fn from_storage() -> Option<Self> {
        let storage = window()?.local_storage().ok()??;
        let json = storage.get_item(STORAGE_KEY).ok()??;
        serde_json::from_str(&json).ok()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string(self) {
            if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                let _ = storage.set_item(STORAGE_KEY, &json);
            }
        }
    }

    fn now() -> NaiveDateTime {
        chrono::Local::now().naive_local()
    }

    pub fn accept_all(version: &str) -> Self {
        let s = Self {
            consent_given: true,
            timestamp: Self::now(),
            version: version.to_string(),
            analytics_allowed: true,
        };
        s.save();
        s
    }

    pub fn accept_necessary(version: &str) -> Self {
        let s = Self {
            consent_given: true,
            timestamp: Self::now(),
            version: version.to_string(),
            analytics_allowed: false,
        };
        s.save();
        s
    }

    pub fn reset(version: &str) -> Self {
        let s = Self {
            version: version.to_string(),
            ..Default::default()
        };
        s.save();
        s
    }
}
