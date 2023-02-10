use crate::prelude::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
static DB_FILE_NAME: &'static str = "quick-auth.db";

pub type ConnPool = Pool<ConnectionManager<SqliteConnection>>;
pub type ConnPooled = PooledConnection<ConnectionManager<SqliteConnection>>;
pub struct Store {
    pub conn: ConnPool,
}
impl Store {
    pub fn new() -> Result<Store> {
        //for production...
        // let mut path = utils::get_app_path(config);
        // path.push(DB_FILE_NAME);
        // println!(
        //     "{}",
        //     &path
        //         .to_str()
        //         .expect("Failed to open database file path.")
        //         .to_string()
        // );
        let manager = ConnectionManager::<SqliteConnection>::new("../test.sqlite");
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create a connection pool.");

        Ok(Store { conn: pool })
    }
}
