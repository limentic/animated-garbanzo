use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi_get_routes, openapi};
use crate::service::housings::*;
use crate::service::addresses::*;
use crate::Db;
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::entity::housings::{NewHousingWithAddress, InsertHousing, SendableHousing};

pub fn load_road(loader : rocket::Rocket<rocket::Build>, settings : &OpenApiSettings) -> rocket::Rocket<rocket::Build> {
    return loader.mount("/api/v1/housings", openapi_get_routes![settings : get_housings, add_housing]);
}

#[openapi]
#[get("/")]
async fn get_housings(conn: Db) -> Json<Vec<SendableHousing>> {
    let result = conn.run(|c| get_list(c)).await;

    match result {
        Ok(list) => return Json(SendableHousing::convert_vec(list)),
        Err(_) => todo!(),
    }
}

#[openapi]
#[post("/", format="json", data="<new_housing>")]
async fn add_housing(conn: Db, new_housing: Json<NewHousingWithAddress>) -> Status {
    let mut housing = new_housing.clone();
    match housing.address{
        Some(address) => {
            let id = conn.run(move |c| create_address(c, &address)).await;
            housing.id_address = Some(id.unwrap() as i32)
        },
        None => todo!(),
    }

    let insertable_housing = InsertHousing{
        id_address: housing.id_address.unwrap(),
        surface: BigDecimal::from_f64(housing.surface).unwrap()
    };
    let result = conn.run(move |c| create_housing(c, &insertable_housing)).await;

    match result{
        Ok(_) => Status::Created,
        Err(err) => {
            println!("{0}", err);
            Status::NotAcceptable
        },
    }
}