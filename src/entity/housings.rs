use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::utils::schema::housings;
use bigdecimal::{BigDecimal, ToPrimitive};

use super::addresses::NewAddress;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Housing {
    pub id: i32,
    pub id_address: i32,
    pub surface: BigDecimal
}

#[derive(Queryable, Serialize, Deserialize, JsonSchema)]
pub struct SendableHousing{
    pub id: i32,
    pub id_address: i32,
    pub surface: f32
}

impl SendableHousing {
    pub fn new(housing : Housing) -> SendableHousing{
        SendableHousing { 
            id:housing.id, 
            id_address: housing.id_address, 
            surface: housing.surface.to_f32().unwrap() }
    }

    pub fn convert_vec(housings : Vec<Housing>) -> Vec<SendableHousing>{
        let mut sendable_housings = Vec::new();
        for housing in housings{
            sendable_housings.push(SendableHousing::new(housing));
        }
        return sendable_housings;
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "housings"]
pub struct InsertHousing {
    pub id_address: i32,
    pub surface: BigDecimal
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct NewHousingWithAddress {
    pub address: Option<NewAddress>,
    pub surface: f64,
    pub id_address: Option<i32>
}