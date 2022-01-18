use diesel_derive_enum::DbEnum;


#[derive(DbEnum, Debug)]
pub enum Role {
    Conseilier,
    Client,
    Admin
}
