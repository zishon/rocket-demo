use rocket::Request;
use rocket_contrib::json::Json;
use crate::response::responder::BaseResponse;

#[catch(422)]
pub fn request_body_error(req: &Request) -> Json<BaseResponse<[i32;0]>> {
    Json(BaseResponse {
        code: 422001,
        message: String::from("请求参数无法被正常解析"),
        data: [0;0],
    })
}
