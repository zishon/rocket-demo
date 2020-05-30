#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use dashmap::DashMap;

use framework::controller::backend;
use framework::controller::frontend;
use framework::middleware::counter::Counter;

fn main() {
    let mut dashmap: DashMap<String, AtomicUsize> = DashMap::new();
    rocket::ignite()
        .attach(Counter::new())
        .manage(dashmap)
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

