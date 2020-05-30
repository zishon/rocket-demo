use std::io::Cursor;

use rocket::http::ContentType;
use rocket::Request;
use rocket::response::{Body, Flash, Redirect, Responder, Response, Result};

pub struct Demo {}

impl<'a> Responder<'a> for Demo {
    fn respond_to(self, _: &Request) -> Result<'a> {
        Response::build()
            .header(ContentType::JSON)
            .raw_body(Body::Sized(Cursor::new("hello from demo"), 15))
            .ok()
    }
}
