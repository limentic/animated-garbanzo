table! {
    use diesel::sql_types::*;
    use crate::utils::role::*;

    addresses (id) {
        id -> Int4,
        city -> Text,
        street -> Text,
        number -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::role::*;

    appointements (id) {
        id -> Int4,
        id_folder -> Int4,
        date -> Timestamp,
        note -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::role::*;

    customers (id) {
        id -> Int4,
        name -> Text,
        surname -> Text,
        email -> Text,
        role -> Nullable<RoleMapping>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::role::*;

    folders (id) {
        id -> Int4,
        folder_num -> Text,
        id_customer -> Int4,
        id_housing -> Int4,
        tax_income -> Numeric,
        situation -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::role::*;

    housings (id) {
        id -> Int4,
        id_address -> Int4,
        surface -> Numeric,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::role::*;

    projects (id) {
        id -> Int4,
        id_folder -> Int4,
        slug -> Nullable<Text>,
        name -> Text,
        reason -> Nullable<Text>,
    }
}

joinable!(appointements -> folders (id_folder));
joinable!(folders -> customers (id_customer));
joinable!(folders -> housings (id_housing));
joinable!(housings -> addresses (id_address));
joinable!(projects -> folders (id_folder));

allow_tables_to_appear_in_same_query!(
    addresses,
    appointements,
    customers,
    folders,
    housings,
    projects,
);
