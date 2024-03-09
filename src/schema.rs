use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    _id: ObjectId,
    pub domain: String,
    pub rank: i32,
    pub scraped: bool,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
}
