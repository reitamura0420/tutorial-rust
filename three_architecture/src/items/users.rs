use crate::presentation::users::*;

pub fn register_users(user: RequestUser) -> bool {
    crate::data_access::users::insert_users(user);
    true
}

pub fn update_disabled(id: i32) -> bool {
    crate::data_access::users::update_disabled(id);
    true
}
