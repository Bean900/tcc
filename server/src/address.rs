use tracing::event;
use uuid::Uuid;

use crate::{
    db::{self, Database},
    error::RestError,
};

#[derive(Debug, Clone)]
pub struct Address {
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Address {
    pub fn from(address: db::models::Address) -> Self {
        Address {
            address: address.address_text,
            latitude: address.latitude,
            longitude: address.longitude,
        }
    }
}

pub fn get_by_id(db: &mut Database, address_id: &Uuid) -> Result<Address, RestError> {
    let address = db.select_address(address_id).map_err(|e| {
        event!(
            tracing::Level::ERROR,
            "Database error while selecting address with id {}: {}",
            address_id,
            e
        );
        RestError::InternalServer {
            message: "Database error while selecting address".to_string(),
        }
    })?;
    Ok(Address::from(address))
}
