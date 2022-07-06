// diesel::table! {
//     tasks (id) {
//         id -> Integer,
//         name -> Varchar,
//         user_id -> Integer,
//         term -> Datetime,
//         update_term_count -> Integer,
//         is_completed -> Bool,
//     }
// }

diesel::table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        user_id -> Integer,
        term -> Datetime,
        update_term_count -> Integer,
        is_completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        mail_address -> Varchar,
        last_name -> Varchar,
        enable -> Bool,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(tasks, users,);
