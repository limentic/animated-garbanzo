use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi_get_routes, openapi};
use crate::service::folders::*;
use crate::Db;

use crate::entity::folders::{NewFolder, SendableFolder, SendableNewFolder};

pub fn load_road(loader : rocket::Rocket<rocket::Build>, settings : &OpenApiSettings) -> rocket::Rocket<rocket::Build> {
    return loader.mount("/api/v1/folders", openapi_get_routes![settings: get_folders, add_folder]);
}

#[openapi]
#[get("/")]
async fn get_folders(conn: Db) -> Json<Vec<SendableFolder>> {
    let result = conn.run(|c| get_list(c)).await;

    match result {
        Ok(list) => {
            let list_sendable = SendableFolder::convert_vec(list);
            return Json(list_sendable)
        },
        Err(_) => todo!(),
    }
}

#[openapi]
#[post("/", format="json", data="<send_new_folder>")]
async fn add_folder(conn: Db, send_new_folder: Json<SendableNewFolder>) -> Status {
    let new_folder = NewFolder::new(&*send_new_folder);
    let result = conn.run(move |c| create_folder(c, &new_folder)).await;

    match result{
        Ok(_) => Status::Created,
        Err(err) => {
            println!("{0}", err);
            Status::NotAcceptable
        },
    }
}