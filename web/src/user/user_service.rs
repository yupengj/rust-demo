use std::ptr::null;
use uuid::Uuid;
use lazy_static::lazy_static;

use crate::user::User;

// lazy_static! {
//     #[derive(Debug)]
//     static ref USER_LIST: Vec<User> = Vec::new();
// }

static mut USER_LIST: Vec<User> = Vec::new();

pub fn find_all() -> Vec<User> {
    unsafe {
        println!("{:#?}", USER_LIST);
        USER_LIST.clone()
    }
}

pub fn find_by_id(id: &str) -> User {
    unsafe {
        println!("{:#?}", USER_LIST);
        for user in USER_LIST.iter() {
            if user.id == id {
                return user.clone();
            }
        }
    }
    User::none()
}

pub fn find_by_name(name: &str) -> User {
    unsafe {
        println!("{:#?}", USER_LIST);
        let mut iter = USER_LIST.iter();
        let option = iter.find(|it| it.name == name);
        match option {
            Some(user) => user.clone(),
            None => User::none()
        }
    }
}

pub fn create_user(mut user: User) -> User {
    unsafe {
        println!("{:#?}", USER_LIST);
        let id = Uuid::new_v4();
        user.id = id.to_string();
        let new_user = user.clone();
        USER_LIST.push(user);
        new_user
    }
}

pub fn update_user(user: User) -> User {
    unsafe {
        println!("{:#?}", USER_LIST);
        let mut iter = USER_LIST.iter_mut();
        let option = iter.find(|it| it.id == user.id);
        match option {
            Some(ul_user) => {
                ul_user.name = user.name;
                ul_user.email = user.email;
                ul_user.password = user.password;
                return ul_user.clone();
            }
            None => User::none()
        }
    }
}

pub fn delete_user(id: &str) -> bool {
    unsafe {
        println!("{:#?}", USER_LIST);
        let mut i = 0;
        while i < USER_LIST.len() {
            let user = &mut USER_LIST[i];
            if user.id == id {
                let val = USER_LIST.remove(i);
                return true;
            } else {
                i += 1;
            }
        }
        false
    }
}
