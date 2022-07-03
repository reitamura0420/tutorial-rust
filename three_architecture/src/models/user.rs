use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: int,
    name: String,
    user_id: int,
    term: String,
    update_term_count: int,
    is_completed: Date,
}

pub mod user;
