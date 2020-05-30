use crate::response::demo::Demo;
use crate::request::api_user::ApiUser;

#[get("/")]
pub fn index() -> String {
    String::from("hello from index")
}

#[get("/demo")]
pub fn demo() -> Demo {
    Demo {}
}

#[get("/counts")]
pub fn counts() {

}

#[get("/api")]
pub fn api(api_user: ApiUser) -> Demo {
    Demo {}
}
/*

#[get("/multi?<id>&<user_id>")]
fn get_multi(id: usize, user_id: usize) -> String {
    format!("id: {}, user_id: {}", id, user_id)
}*/
