use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes};
use crate::entity::projects::Project;
use crate::entity::projects::NewProject;
use crate::service::projects::*;
use crate::Db;

pub fn load_road(loader : rocket::Rocket<rocket::Build>, settings : &OpenApiSettings) -> rocket::Rocket<rocket::Build> {
    return loader.mount("/api/v1/projects", openapi_get_routes![settings : get_projects, add_project]);
}

#[openapi]
#[get("/")]
async fn get_projects(conn: Db) -> Json<Vec<Project>> {
    let result = conn.run(|c| get_list(c)).await;

    match result {
        Ok(list) => return Json(list),
        Err(_) => todo!(),
    }
}

#[openapi]
#[post("/", format="json", data="<new_project>")]
async fn add_project(conn: Db, new_project: Json<NewProject>) -> Status {
    let result = conn.run(move |c| create_project(c, &*new_project)).await;

    match result{
        Ok(_) => Status::Created,
        Err(err) => {
            println!("{0}", err);
            Status::NotAcceptable
        },
    }
}
