/*#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct SampleResponder {
    inner: OtherResponder,
    header: ContentType,
    more: Header<'static>,
    #[response(ignore)]
    unrelated: MyType,
}*/
use serde::Serialize;
use serde::Deserialize;
use rocket::response::Responder;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}
