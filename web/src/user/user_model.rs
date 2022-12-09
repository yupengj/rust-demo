use poem_openapi::{Object};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct User {
    #[oai(read_only)]
    pub id: String,
    #[oai(validator(max_length = 64))]
    pub name: String,
    #[oai(validator(max_length = 32))]
    pub password: String,
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
    // pub fn new(id: String, name: String, password: Password, email: Email) -> User {
    //     User {
    //         id,
    //         name,
    //         password,
    //         email,
    //     }
    // }
}
