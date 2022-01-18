use diesel::PgConnection;
use crate::utils::schema::addresses;
use crate::diesel::RunQueryDsl;
use crate::entity::addresses::NewAddress;



pub fn create_address(conn : &PgConnection, new_address: &NewAddress) -> Result<usize, diesel::result::Error>{
    return diesel::insert_into(addresses::table).values(new_address).execute(conn);
}