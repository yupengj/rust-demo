use poem::IntoResponse;
use poem_openapi::{
    param::Path,
    payload::Json,
    types::{Email, Password}, Object, OpenApi, OpenApiService, Tags,
};
use poem_openapi::__private::serde_json::json;
use poem_openapi::payload::PlainText;
use slab::Slab;
use tokio::sync::Mutex;

use crate::user::user_model;
use crate::user::user_service;
use user_model::User;
use user_service::{create_user, delete_user, find_all, find_by_id, update_user};
use crate::Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }

    /// 创建用户
    #[oai(path = "/user/create", method = "post")]
    async fn create_user(&self, user: Json<User>) -> Json<User> {
        let create_user = create_user(user.0);
        Json(create_user)
    }
    /// 根据id查询用户
    #[oai(path = "/user/findById", method = "post")]
    async fn find_user(&self, user_id: Json<String>) -> Json<User> {
        Json(find_by_id(user_id.0))
    }

    /// 更新用户
    #[oai(path = "/user/update", method = "post")]
    async fn update_user(&self, user: Json<User>) -> Json<User> {
        Json(update_user(user.0))
    }

    /// 删除用户
    #[oai(path = "/user/delete", method = "post")]
    async fn delete_user(&self, user_id: Json<String>) -> Json<bool> {
        Json(delete_user(user_id.0))
    }
}
