use poem_openapi::{
    types::{Email, Password},
    ApiResponse, Object, OpenApi, OpenApiService, Tags,
};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct User {
    #[oai(read_only)]
    pub id: i64,
    #[oai(validator(max_length = 64))]
    pub name: String,
    #[oai(validator(max_length = 32))]
    pub password: Password,
    pub email: Email,
}
