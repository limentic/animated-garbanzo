#[macro_use]
extern crate diesel;
use rocket::{Rocket, Build};
use rocket_okapi::{request::{OpenApiFromRequest, RequestHeaderInput}, gen::OpenApiGenerator, swagger_ui::{make_swagger_ui, SwaggerUIConfig}, rapidoc::{RapiDocConfig, make_rapidoc, GeneralConfig, HideShowConfig, UiConfig, Theme}, settings::UrlObject};
use rocket_sync_db_pools::{database};
mod entity;
mod service;
mod controller;
mod utils;

#[database("main_db")]
struct Db(diesel::PgConnection);

fn load_road(loader : Rocket<Build>) -> Rocket<Build>{
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    let loader = controller::status::load_road(loader);
    let loader = controller::v1::housings::load_road(loader, &settings);
    let loader = controller::v1::projects::load_road(loader, &settings);
    let loader = controller::v1::folders::load_road(loader, &settings);

    return loader;
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build().attach(Db::fairing());
    let loader = load_road(loader);

    // La macro vec! permet de créer un vec depuis une liste, pour l'instant on le crée manuéllement a voire pour la suite
    let urls = vec![
        UrlObject::new("folders", "/api/v1/folders/openapi.json"),
        UrlObject::new("housings", "/api/v1/housings/openapi.json"),
        UrlObject::new("projects", "/api/v1/projects/openapi.json")
    ];

    let loader = loader.mount(
        "/doc/swagger",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/api/v1/folders/openapi.json".to_owned(),
            urls : urls.clone(),
            ..Default::default()
        }),
    ).mount("/doc/rapidoc", make_rapidoc(&RapiDocConfig{
        general: GeneralConfig {
            spec_urls: urls,
            ..Default::default()
        },
        ui : UiConfig{
            theme: Theme::Dark,
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    }));


    loader.launch().await
}

impl<'r> OpenApiFromRequest<'r> for Db {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}