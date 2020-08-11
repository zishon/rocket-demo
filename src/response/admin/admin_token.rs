use std::io::Cursor;

use rocket::http::ContentType;
use rocket::Request;
use rocket::response::{Body, Flash, Redirect, Responder, Response, Result};
use serde::Serialize;
use serde::Deserialize;

#[derive(Responder)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct AdminToken {
    pub token: String,
}
