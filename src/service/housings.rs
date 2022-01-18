use diesel::PgConnection;
use crate::utils::schema::housings;
use crate::diesel::RunQueryDsl;
use crate::utils::schema::housings::dsl::*;
use diesel::result::Error;


use crate::entity::housings::{Housing, InsertHousing};


pub fn get_list(conn : &PgConnection) -> Result<Vec<Housing>, Error> {
    housings.load::<Housing>(conn)
}

pub fn create_housing(conn : &PgConnection, new_housing: &InsertHousing) -> Result<usize, diesel::result::Error>{
    return diesel::insert_into(housings::table).values(new_housing).execute(conn);
}
