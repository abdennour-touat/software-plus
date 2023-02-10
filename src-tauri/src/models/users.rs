use crate::{
    db::ConnPooled,
    deleteAll, insert,
    schema::user::{self, dsl::*},
    utils::error::Error,
};
use diesel::{QueryDsl, RunQueryDsl, SqliteExpressionMethods};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::key::KeyBmc;
use crate::prelude::Result;

#[derive(Debug, TS, Serialize)]
#[ts(export, export_to = "../src/bindings/")]
pub enum AuthData {
    NoLicense,
    NoUser,
    BadData,
    Valid,
}

#[derive(Deserialize, Debug, TS, Queryable, Insertable, Serialize)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug, TS, Queryable, Insertable, Serialize)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name = user)]
pub struct NewUser {
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
    pub fn insert(store: &mut ConnPooled, data: NewUser) -> Result<()> {
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
                Ok(NewUser {
                    username: data.username,
                    password: hashed_password,
                })
            }
            None => Err(Error::CtxFail),
        };
        return match data {
            Ok(val) => {
                insert!(user)
                    .values(val)
                    // .returning(id)
                    .execute(store)
                    .expect("couldn't insert user");
                Ok(())
            }
            Err(e) => Err(e),
        };
    }
    pub fn get_user(store: &mut ConnPooled, data: i32) -> Result<Vec<User>> {
        let res: std::result::Result<Vec<User>, diesel::result::Error> =
            user.filter(id.is(data)).load(store);
        match res {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::CtxFail),
        }
    }
    pub fn get_users(store: &mut ConnPooled) -> Result<Vec<User>> {
        return match user.load(store) {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::CtxFail),
        };
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
            Some(val) => Ok(String::from(decrypt(&mcrypt, val.password.as_str()))),
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
        let res = match res {
            Ok(val) => Ok(val),
            Err(err) => Ok(err),
        };
        res
    }
    pub fn delete_all(store: &mut ConnPooled) -> Result<bool> {
        match deleteAll!(user).execute(store) {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::CtxFail),
        }
    }
}
