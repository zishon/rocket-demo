#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use] extern crate rocket;

pub mod middleware {
    pub mod counter;
}
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
    pub mod sample;
    pub mod demo;
}

pub mod db {
    pub mod app;
}
