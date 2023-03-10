use serde::Serialize;
use ts_rs::TS;

pub mod key;
pub mod videos;

pub mod users;
#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct ModelMutateResultData {
    pub id: String,
}

impl From<String> for ModelMutateResultData {
    fn from(id: String) -> Self {
        Self { id }
    }
}
