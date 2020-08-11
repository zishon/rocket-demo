use crate::response::demo::Demo;
use crate::request::api_user::ApiUser;
use rocket_contrib::databases::diesel;
use diesel::prelude::*;
use crate::models::{ApiUserSecret, InsertableApiUserSecret};
use crate::schema::*;
use std::error::Error;
use crate::connection::*;
use serde::Serialize;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(connection: MysqlDatabase) -> String {
    let record = InsertableApiUserSecret {
        app_code: "app_code",
        app_secret: "app_secret",
    };
    let result = diesel::insert_into(api_user_secret::table)
        .values(&record)
        .execute(&(connection.0));
    println!("{:?}", result.unwrap());
    String::from("hello")
}

#[get("/demo")]
pub fn demo() -> Json<Demo> {
    Json(Demo {
        id: 1,
        title: String::from("hello"),
    })
}

#[get("/counts")]
pub fn counts() {

}

#[get("/api")]
pub fn api(api_user: ApiUser) -> String {
    String::from("hello")
}
/*

#[get("/multi?<id>&<user_id>")]
fn get_multi(id: usize, user_id: usize) -> String {
    format!("id: {}, user_id: {}", id, user_id)
}*/
