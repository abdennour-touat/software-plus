use crate::{
    db::ConnPooled,
    delete, deleteAll, insert,
    schema::user_table::{self, dsl::*},
    utils::error::{AuthData, Error},
};
use diesel::{QueryDsl, RunQueryDsl, SqliteExpressionMethods};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::key::KeyBmc;
use crate::prelude::Result;

#[derive(Deserialize, Debug, TS, Queryable, Insertable, Serialize)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name = user_table)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug, TS, Queryable, Insertable, Serialize)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name = user_table)]
pub struct UserForCreate {
    pub username: String,
    pub password: String,
}
pub struct UserBmc;

//helper function to abstract a bit
fn encrypt(crypt: &MagicCrypt256, string: &str) -> String {
    crypt.encrypt_str_to_base64(string)
}
fn decrypt(crypt: &MagicCrypt256, string: &str) -> String {
    crypt
        .decrypt_base64_to_string(string)
        .expect("error decrypting")
}
impl UserBmc {
    pub fn insert(store: &mut ConnPooled, data: UserForCreate) -> Result<()> {
        //initialze the encrypt
        let mcrypt = new_magic_crypt!("magickey", 256);
        //get the key
        let key = KeyBmc::get(store);
        let data = match key?.get(0) {
            Some(val) => {
                //concat the password with the password and encrypt it
                let hashed_password = encrypt(
                    &mcrypt,
                    &format!(
                        "{}{}",
                        &mcrypt
                            .decrypt_base64_to_string(&val.hash)
                            .expect("error dycripting")[0..11],
                        data.password.as_str()
                    ),
                );
                Ok(UserForCreate {
                    username: data.username,
                    password: hashed_password,
                })
            }
            None => Err(Error::DataBaseError("No key found".to_string())),
        };
        return match data {
            Ok(val) => {
                insert!(user_table)
                    .values(val)
                    // .returning(id)
                    .execute(store)
                    .expect("couldn't insert user");
                Ok(())
            }
            Err(err) => Err(err),
        };
    }
    pub fn get_user(store: &mut ConnPooled, data: i32) -> Result<Vec<User>> {
        let res: std::result::Result<Vec<User>, diesel::result::Error> =
            user_table.filter(id.is(data)).load(store);
        match res {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::DataBaseError("No user found".to_string())),
        }
    }
    pub fn get_users(store: &mut ConnPooled) -> Result<Vec<User>> {
        match user_table.load(store) {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::DataBaseError("No user found".to_string())),
        }
    }
    pub fn check_authentication(store: &mut ConnPooled) -> Result<AuthData> {
        //initialize the encrypt
        let mcrypt = new_magic_crypt!("magickey", 256);
        //get the first user..
        let pd = UserBmc::get_users(store);
        //get the key
        let binding = KeyBmc::get(store);
        //decrypt the key
        let key = match binding?.get(0) {
            Some(val) => Ok(decrypt(&mcrypt, &val.hash)),
            None => Err(AuthData::NoLicense),
        };
        //decrypt the password
        let auth = match pd?.get(0) {
            Some(val) => Ok(decrypt(&mcrypt, val.password.as_str())),
            None => Err(AuthData::NoUser),
        };
        let res = match key {
            Ok(key_val) => match auth {
                Ok(val) => match val[0..11].eq(&key_val[0..11]) {
                    true => Ok(AuthData::Valid),
                    false => Err(AuthData::BadData),
                },
                Err(_) => Err(AuthData::NoUser),
            },
            Err(_) => Err(AuthData::NoLicense),
        };
        match res {
            Ok(val) => Ok(val),
            Err(err) => Ok(err),
        }
    }
    pub fn delete(store: &mut ConnPooled, user_id: i32) -> Result<()> {
        match delete!(user_table, id.is(user_id)).execute(store) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::DataBaseError("couldn't delete user".to_string())),
        }
        // .expect("couldn't delete user");
    }
    pub fn delete_all(store: &mut ConnPooled) -> Result<bool> {
        match deleteAll!(user_table).execute(store) {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::DataBaseError("couldn't delete users".to_string())),
        }
    }
}
