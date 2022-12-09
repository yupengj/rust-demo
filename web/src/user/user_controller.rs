use poem_openapi::{
    param::Path,
    payload::Json,
    types::{Email, Password},
    ApiResponse, Object, OpenApi, OpenApiService, Tags,
};
use slab::Slab;
use tokio::sync::Mutex;

use crate::user::user_model;
use crate::user::user_service;
use user_model::User;
use user_service::{create_user, delete_user, find_all, find_by_id, update_user};

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

#[OpenApi]
impl Api {
    #[oai(path = "/users/create", method = "post", tag = "ApiTags::User")]
    async fn create_user(&self, user: Json<User>) -> User {
        create_user(user.0)
    }

    #[oai(path = "/users/findById", method = "post", tag = "ApiTags::User")]
    async fn find_user(&self, user_id: Json<String>) -> User {
        find_by_id(user_id.0)
    }

    #[oai(path = "/users/update", method = "post", tag = "ApiTags::User")]
    async fn update_user(&self, user: Json<User>) -> User {
        update_user(user.0)
    }

    #[oai(path = "/users/delete", method = "post", tag = "ApiTags::User")]
    async fn delete_user(&self, user_id: Json<String>) -> bool {
        delete_user(user_id.0)
    }
}
