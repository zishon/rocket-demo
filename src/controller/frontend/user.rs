use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};
use crate::request::login_user::User;

#[get("/login")]
pub fn login(mut cookies: Cookies) -> std::result::Result<Redirect, Flash<Redirect>> {
    //获取用户
    let user = User {
        name: String::from("login user"),
    };
    cookies.add_private(Cookie::new("login_user", user.name));
    //println!("{:?}", request.cookies().get_private("token"));
    Ok(Redirect::to(uri!(super::index::index)))
}

#[get("/user/home")]
pub fn home(login_user: User) -> String {
    let mut s = String::from("hello from login user:");
    s + login_user.name.as_str()
}


/*

#[get("/multi?<id>&<user_id>")]
fn get_multi(id: usize, user_id: usize) -> String {
    format!("id: {}, user_id: {}", id, user_id)
}*/
