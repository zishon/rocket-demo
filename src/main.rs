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

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use dashmap::DashMap;

use framework::controller::backend;
use framework::controller::frontend;
use framework::middleware::counter::Counter;
use framework::connection::*;
use rocket_contrib::databases::diesel;
use framework::error::catcher;

fn main() {
    let mut dashmap: DashMap<String, AtomicUsize> = DashMap::new();
    rocket::ignite()
        .attach(Counter::new())
        .attach(MysqlDatabase::fairing())
        .manage(dashmap)
        .register(catchers![catcher::request_body_error])
        .mount("/", routes![
            backend::index::admin,
            backend::index::login,
            frontend::user::login,
            frontend::user::home,
            frontend::index::index,
            frontend::index::demo,
            frontend::index::counts,
            frontend::index::api
        ])
        .launch();
}


