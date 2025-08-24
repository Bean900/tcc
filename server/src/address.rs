use uuid::Uuid;

use crate::db::{self, Database};

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

pub fn get_by_id(db: &mut Database, address_id: &Uuid) -> Result<Address, String> {
    let address = db.select_address(address_id)?;
    Ok(Address::from(address))
}
