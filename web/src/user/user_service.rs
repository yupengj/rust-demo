use uuid::Uuid;
use std::ptr::null;
use crate::user::User;

const USER_LIST: Vec<User> = Vec::new();

pub fn find_all() -> Vec<User> {
    USER_LIST.clone()
}

pub fn find_by_id(id: String) -> User {
    let option = USER_LIST.iter().find(|it| it.id == id);
    return option[0];
}

pub fn find_by_name(name: String) -> User {
    let option = USER_LIST.iter().find(|it| it.name == name);
    return option[0];
}

pub fn create_user(&mut user: User) -> User {
    let id = Uuid::new_v4();
    user.id = id;
    USER_LIST.push(user);
    user.clone()
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