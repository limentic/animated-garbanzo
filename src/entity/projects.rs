use rocket_okapi::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::utils::schema::projects;

#[derive(Queryable, Serialize, Deserialize, JsonSchema)]
pub struct Project {
    pub id: i32,
    pub id_folder: i32,
    pub slug: Option<String>,
    pub name: String,
    pub reason: Option<String>
}

#[derive(Insertable, Serialize, Deserialize, JsonSchema)]
#[table_name = "projects"]
pub struct NewProject {
    pub id_folder: i32,
    pub slug: Option<String>,
    pub name: String,
    pub reason: Option<String>
}