use rocket::{get, routes};
use rocket::http::Status;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    return loader.mount("/", routes![status, quickstart, coffemaker]);
}

#[get("/")]
async fn status() -> &'static str {
    return "ok"
}

#[get("/quickstart")]
async fn quickstart() -> &'static str {
    return "Wow, that's was quick!"
}

#[get("/coffemaker")]
async fn coffemaker() -> Status {
    Status::ImATeapot
}