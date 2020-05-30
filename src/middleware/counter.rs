use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};

use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};

#[derive(Debug)]
pub struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            get: AtomicUsize::new(0),
            post: AtomicUsize::new(0),
        }
    }
}

impl Fairing for Counter {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    fn on_request(&self, request: &mut Request, _: &Data) {
        //以下部分也可以替换为写入redis，mysql等操作
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _ => return
        };
        println!("{:?}", self);
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        // Don't change a successful user's response, ever.
        if response.status() != Status::Ok {
            return
        }

        // Rewrite the response to return the current counts.
        if request.method() == Method::Get && request.uri().path() == "/counts" {
            let get_count = self.get.load(Ordering::Relaxed);
            let post_count = self.post.load(Ordering::Relaxed);
            let body = format!("Get: {}\nPost: {}", get_count, post_count);

            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(body));
        }
    }
}
