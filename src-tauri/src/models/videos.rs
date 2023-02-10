use crate::db::ConnPooled;
use crate::schema::video::dsl::*;
use crate::schema::video::{self};
use crate::{delete, insert, update};
use diesel::{QueryDsl, RunQueryDsl, SqliteExpressionMethods};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Insertable, TS, Deserialize)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name = video)]
pub struct NewVideo {
    pub title: String,
    pub description: String,
    pub removed: bool,
}

#[derive(Queryable, Debug, AsChangeset, Serialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name = video)]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub removed: bool,
}
#[derive(Queryable, Debug, AsChangeset, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
#[diesel(table_name = video)]
pub struct VideoForUpdate {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub removed: bool,
}
#[derive(Queryable, Debug, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct VideoForDelete {
    pub id: i32,
}
pub struct VideoBmc;
impl VideoBmc {
    pub fn insert(store: &mut ConnPooled, data: NewVideo) {
        insert!(video)
            .values(data)
            .execute(store)
            .expect("couldn't insert");
    }
    pub fn update(store: &mut ConnPooled, rid: i32, data: VideoForUpdate) {
        update!(video, id.is(rid))
            .set(data)
            .execute(store)
            .expect("couldn't update the tupel");
    }
    pub fn delete(store: &mut ConnPooled, rid: i32) {
        delete!(video, id.is(rid))
            .execute(store)
            .expect("couldn't delete");
    }
    pub fn list(store: &mut ConnPooled) -> Vec<Video> {
        video.load(store).expect("couldn't load tupels")
    }
}
