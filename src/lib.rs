#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate chrono;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;


//use rocket_contrib::databases::diesel;
use diesel::prelude::*;

pub mod middleware {
    pub mod counter;
}

pub mod connection;

pub mod request {
    pub mod admin_user;
    pub mod login_user;
    pub mod api_user;
}
pub mod controller {
    pub mod frontend;
    pub mod backend;
}
pub mod response {
    pub mod responder;
    pub mod demo;
    pub mod admin {
        pub mod admin_token;
    }
}

pub mod db {
    pub mod app;
}

pub mod provider {
    pub mod admin;
    pub mod admin_access_token;
}

pub mod error {
    pub mod catcher;
}

pub mod schema;
pub mod models;
