use serde::Serialize;
use ts_rs::TS;

pub mod key;

pub mod users;
#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../src/bindings/")]
pub struct ModelMutateResultData {
    pub id: String,
}

impl From<String> for ModelMutateResultData {
    fn from(id: String) -> Self {
        Self { id }
    }
}
