use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};

use crate::request::admin_user::AdminUser;
use crate::request::admin_user::AdminLogin;
use crate::request::login_user::User;
use serde::Deserialize;
use crate::connection::*;
use crate::provider::admin_provider;
use crate::provider::admin_access_token_provider;
use crate::provider::last_insert_provider::*;
use rocket_contrib::json::Json;
use crate::response::responder::BaseResponse;
use crate::response::admin::admin_token::AdminToken;
use diesel::result::Error;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::schema::*;
use crate::models::*;
use rocket_contrib::databases::diesel;
use diesel::prelude::*;

#[get("/admin")]
pub fn admin(admin_user: AdminUser) -> String {
    let mut s = String::from("from admin user");
    s + admin_user.name.as_str()
}

/*#[get("/admin/login")]
pub fn login(mut cookies: Cookies) -> std::result::Result<Redirect, Flash<Redirect>> {
    //获取用户
    let admin_user = AdminUser {
        name: String::from("admin user:"),
    };
    cookies.add_private(Cookie::new("admin_user", admin_user.name));
    //println!("{:?}", request.cookies().get_private("token"));
    Ok(Redirect::to(uri!(super::index::admin)))
}*/

fn md5(s: &str) -> String {
    format!("{:x}", md5::compute(s))
}

fn passwd_hash(p: &str, s: &str) -> String {
    let without_salt_hash = format!("{:?}", md5::compute(p));
    format!("{:x}", md5::compute((without_salt_hash + s).as_str()))
}

fn insert_access_token(access_token: &str, admin_id: i32, connection: &MysqlDatabase) -> bool {
    let record = InsertableAdminAccessToken {
        admin_id: admin_id,
        access_token: access_token,
    };
    let result = diesel::insert_into(admin_access_token::table)
        .values(&record)
        .execute(&connection.0);
    true
}

#[post("/admin/login", format = "application/json", data = "<admin_login>")]
pub fn login(admin_login: Json<AdminLogin>, connection: MysqlDatabase) -> Result<Json<BaseResponse<AdminToken>>, Error> {
    let admin_record = admin_provider::get_by_name(&admin_login.name, &connection);
    if let Ok(admin_record) = admin_record {
        if passwd_hash(&admin_login.password, &admin_record.salt) != admin_record.passwd {
            return Err(Error::NotFound);
        }
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .collect();
        insert_access_token(&rand_string, admin_record.id, &connection);
        let admin_token = AdminToken {
            token: rand_string,
        };
        Ok(Json(BaseResponse {
            code: 0,
            message: String::new(),
            data: admin_token,
        }))
    } else {
        Err(Error::NotFound)
    }
    //判断admin是否存在
    //如果admin存在，则生成一个access token，并保存到admin_access_token表里
    //返回access_token，响应内容为一个json
}
