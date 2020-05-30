use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};

use crate::request::admin_user::AdminUser;
use crate::request::login_user::User;

#[get("/admin")]
pub fn admin(admin_user: AdminUser) -> String {
    let mut s = String::from("from admin user");
    s + admin_user.name.as_str()
}

#[get("/admin/login")]
pub fn login(mut cookies: Cookies) -> std::result::Result<Redirect, Flash<Redirect>> {
    //获取用户
    let admin_user = AdminUser {
        name: String::from("admin user:"),
    };
    cookies.add_private(Cookie::new("admin_user", admin_user.name));
    //println!("{:?}", request.cookies().get_private("token"));
    Ok(Redirect::to(uri!(super::index::admin)))
}
