use diesel::PgConnection;
use crate::utils::schema::projects;
use crate::diesel::RunQueryDsl;
use crate::utils::schema::projects::dsl::*;
use diesel::result::Error;

use crate::entity::projects::{Project, NewProject};

pub fn create_project(conn : &PgConnection, new_project: &NewProject) -> Result<usize, diesel::result::Error>{
    return diesel::insert_into(projects::table).values(new_project).execute(conn);
}

pub fn get_list(conn : &PgConnection) -> Result<Vec<Project>, Error> {
    projects.load::<Project>(conn)
}