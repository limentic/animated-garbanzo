use serde::{Serialize, Deserialize};
use crate::utils::schema::addresses;
use schemars::JsonSchema;


#[derive(Queryable, Serialize, Deserialize, JsonSchema)]
pub struct Address {
    pub id: i32,
    pub city: String,
    pub street: String,
    pub number: String
}

#[derive(Insertable, Serialize, Deserialize, Clone, JsonSchema)]
#[table_name = "addresses"]
pub struct NewAddress {
    pub city: String,
    pub street: String,
    pub number: String
}