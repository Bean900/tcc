use std::{
    fs::File,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use serde_derive::Deserialize;

use rfd::FileDialog;
#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct Contact {
    pub id: u8,
    pub team_name: String,
    pub address: String,
    pub latitude: i32,
    pub longitude: i32,
}
#[derive(Deserialize)]
pub struct ContactInternal {
    pub id: u8,
    pub team_name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Contact {
    pub fn new(id: u8, team_name: &str, address: &str, latitude: f64, longitude: f64) -> Self {
        Contact {
            id,
            team_name: team_name.to_string(),
            address: address.to_string(),
            latitude: f64_to_i32(latitude),
            longitude: f64_to_i32(longitude),
        }
    }
}

fn f64_to_i32(value: f64) -> i32 {
    (value * 10f64.powi(5)).round() as i32
}

pub(crate) struct ContactLoader {}

impl ContactLoader {
    pub(crate) fn new() -> Self {
        ContactLoader {}
    }

    pub(crate) fn load(&self) -> Result<Option<Vec<Contact>>, Error> {
        let files = FileDialog::new()
            .add_filter("CSV-File", &["csv"])
            .pick_file();

        match files {
            Some(path_buf) => match self.read_file(path_buf) {
                Ok(data) => Ok(Some(data)),
                Err(err) => Err(err),
            },
            None => Ok(None),
        }
    }

    fn read_file(&self, path_buf: PathBuf) -> Result<Vec<Contact>, Error> {
        if path_buf.is_dir() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Selected Path is not a CSV file!",
            ));
        }
        let file = File::open(path_buf.as_path()).map_err(|err| {
            println!("Error while opening CSV file: {err}");
            return Error::new(ErrorKind::InvalidData, "Error while opening CSV file!");
        })?;

        let mut rdr = csv::Reader::from_reader(file);

        let mut iter = rdr.deserialize();

        let mut contact_list: Vec<Contact> = Vec::new();
        let mut index = 0_u8;
        loop {
            if let Some(result) = iter.next() {
                let mut contact: ContactInternal = result.map_err(|err| {
                    println!("Error while mapping CSV data: {err}");
                    return Error::new(ErrorKind::InvalidData, "Error while mapping CSV data!");
                })?;
                contact.id = index;
                contact_list.push(Contact::new(
                    contact.id,
                    contact.team_name.as_str(),
                    contact.address.as_str(),
                    contact.latitude,
                    contact.longitude,
                ));
                index += 1;
            } else {
                break;
            }
        }
        return Ok(contact_list);
    }
}
