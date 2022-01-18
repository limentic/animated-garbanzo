use diesel::PgConnection;
use crate::utils::schema::folders;
use crate::diesel::RunQueryDsl;
use crate::utils::schema::folders::dsl::*;
use diesel::result::Error;

use crate::entity::folders::{Folder, NewFolder};

pub fn create_folder(conn : &PgConnection, new_folder: &NewFolder) -> Result<usize, diesel::result::Error>{
    return diesel::insert_into(folders::table).values(new_folder).execute(conn);
}

pub fn get_list(conn : &PgConnection) -> Result<Vec<Folder>, Error> {
    folders.load::<Folder>(conn)
}