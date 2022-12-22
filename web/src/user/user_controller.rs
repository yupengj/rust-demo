use poem::IntoResponse;
use poem_openapi::{
    param::Path,
    payload::Json,
    types::{Email, Password}, Object, OpenApi, OpenApiService, Tags,
};
use poem_openapi::__private::serde_json::{json, Value};
use poem_openapi::payload::PlainText;
use poem_openapi::types::ToJSON;
use slab::Slab;
use tokio::sync::Mutex;

use crate::user::user_model;
use crate::user::user_service;
use user_model::User;
use crate::Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World. /api into api")
    }

    /// 查询所有用户
    #[oai(path = "/user/findAll", method = "post")]
    async fn find_all(&self) -> Json<Vec<User>> {
        Json(user_service::find_all())
    }

    /// 根据id查询用户
    #[oai(path = "/user/findById", method = "post")]
    async fn find_user_by_id(&self, user_id: Json<String>) -> Json<User> {
        let temp_value = user_id.to_json().unwrap();
        let id = temp_value.as_str().unwrap();
        let user = user_service::find_by_id(id);
        Json(user)
    }

    /// 根据用户名查询用户
    #[oai(path = "/user/findByName", method = "post")]
    async fn find_user_by_name(&self, name: Json<String>) -> Json<User> {
        let temp_value = name.to_json().unwrap();
        let name = temp_value.as_str().unwrap();
        let user = user_service::find_by_name(name);
        Json(user)
    }

    /// 创建用户
    #[oai(path = "/user/create", method = "post")]
    async fn create_user(&self, user: Json<User>) -> Json<User> {
        let create_user = user_service::create_user(user.0);
        Json(create_user)
    }

    /// 更新用户
    #[oai(path = "/user/update", method = "post")]
    async fn update_user(&self, user: Json<User>) -> Json<User> {
        Json(user_service::update_user(user.0))
    }

    /// 删除用户
    #[oai(path = "/user/delete", method = "post")]
    async fn delete_user(&self, user_id: Json<String>) -> Json<bool> {
        let temp_value = user_id.to_json().unwrap();
        let id = temp_value.as_str().unwrap();
        Json(user_service::delete_user(id))
    }
}
