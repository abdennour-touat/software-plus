use crate::deleteAll;
use crate::prelude::Result;
use crate::schema::license_key;
use crate::schema::license_key::dsl::*;
use crate::utils::error::Error;
use crate::utils::keyverifier::KeyVerifer;
use crate::{db::ConnPooled, insert};
use ::license_key::Status;
use diesel::prelude::*;
use magic_crypt::MagicCryptTrait;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
// TODO finish the crud logic
// TODO run tests
// TODO update use to users

#[derive(Serialize, Deserialize, Debug, TS, Queryable)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Key {
    pub id: i32,
    pub hash: String,
}

#[derive(Debug, Deserialize, TS, Insertable)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name= license_key)]
pub struct KeyForCreate {
    pub hash: String,
}

pub struct KeyBmc;
impl KeyBmc {
    pub fn insert(store: &mut ConnPooled, data: KeyForCreate) -> Status {
        let res = match KeyVerifer::verify(&data.hash) {
            Ok(o) => {
                // check if the key is valid
                let mcrypt = new_magic_crypt!("magickey", 256);
                //hash the key before storing it ...
                let new_key = KeyForCreate {
                    hash: mcrypt.encrypt_str_to_base64(&data.hash),
                };
                insert!(license_key)
                    .values(new_key)
                    .execute(store)
                    .expect("couldn't insert the key");
                o
            }
            Err(e) => e,
        };
        res
    }
    pub fn get(store: &mut ConnPooled) -> Result<Vec<Key>> {
        match license_key.load(store) {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::DataBaseError("couldn't get the key".to_string())),
        }
    }
    pub fn delete_all(store: &mut ConnPooled) -> Result<bool> {
        match deleteAll!(license_key).execute(store) {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::DataBaseError("couldn't delete the key".to_string())),
        }
    }
}
