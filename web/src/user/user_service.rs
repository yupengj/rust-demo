use crate::user::User;
use std::ptr::null;
use uuid::Uuid;

const USER_LIST: Vec<User> = Vec::new();

pub fn find_all() -> Vec<User> {
    USER_LIST.clone()
}

pub fn find_by_id(id: String) -> User {
    let option = USER_LIST.iter().find(|it| it.id == id);
    User::new()
}

pub fn find_by_name(name: String) -> User {
    let option = USER_LIST.iter().find(|it| it.name == name);
    User::new()
}

pub fn create_user(mut user: User) -> User {
    let id = Uuid::new_v4();
    user.id = id.to_string();
    // USER_LIST.push(new_user);
    // new_user.clone()
    user
}

pub fn update_user(user: User) -> User {
    //USER_LIST.drain(|&mut it| it.id == user.id);
    //USER_LIST.push(user);
    user.clone()
}

pub fn delete_user(id: String) -> bool {
    // USER_LIST.drain(|&mut it| it.id == user.id);
    true
}
