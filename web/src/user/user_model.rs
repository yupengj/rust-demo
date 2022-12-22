use poem_openapi::{Object, ApiResponse};

// #[derive(Debug, Object, Clone, Eq, PartialEq)]
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct User {
    #[oai]
    pub id: String,
    #[oai(validator(max_length = 64))]
    pub name: String,
    #[oai(validator(max_length = 32))]
    pub password: String,
    #[oai(validator(max_length = 64))]
    pub email: String,
}

impl User {
    pub fn new() -> User {
        User {
            id: String::from("123"),
            name: String::from("张三"),
            password: String::from("123"),
            email: String::from("123"),
        }
    }
    pub fn none() -> User {
        User {
            id: String::from("不存在用户"),
            name: String::from("不存在用户"),
            password: String::from("不存在用户"),
            email: String::from("不存在用户"),
        }
    }
}
