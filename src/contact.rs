use std::{
    fs::File,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use serde::Deserialize;

use rfd::FileDialog;
#[derive(PartialEq, Debug, Deserialize, Clone, Eq, Hash)]
pub(crate) struct Contact {
    pub team_name: String,
    pub address: String,
    pub latitude: String,
    pub longitude: String,
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
        loop {
            if let Some(result) = iter.next() {
                let contact: Contact = result.map_err(|err| {
                    println!("Error while mapping CSV data: {err}");
                    return Error::new(ErrorKind::InvalidData, "Error while mapping CSV data!");
                })?;
                contact_list.push(contact);
            } else {
                break;
            }
        }
        return Ok(contact_list);
    }
}
