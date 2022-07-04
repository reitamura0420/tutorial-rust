#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use diesel::table;
    tasks (id) {
        id -> Integer,
        name -> Text,
        user_id -> Integer,
        term -> Varchar,
        update_term_count -> Integer,
        is_completed -> Bool,
    }
}

// table! {
//     users (id) {
//         id -> Integer,
//         mail_address -> Varchar,
//         last_name -> Varchar,
//         enable -> Bool,
//     }
// }

// joinable!(tasks -> users (user_id));

// allow_tables_to_appear_in_same_query!(tasks, users,);
