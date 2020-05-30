use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use dashmap::{DashMap, Map};
use rocket::outcome::IntoOutcome;
use rocket::{Request, State};
use rocket::request::FromRequest;

#[derive(Debug)]
pub struct ApiUser {
    pub name: String,
}

#[derive(Debug)]
pub enum ApiUserError {
    Invalid,
}



impl<'a, 'r> FromRequest<'a, 'r> for ApiUser {
    type Error = ApiUserError;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let app_id: String = request.get_query_value("app_id").unwrap().unwrap();
        let sign: String = request.get_query_value("sign").unwrap().unwrap();


        let mut dash_map: State<DashMap<String, AtomicUsize>> = request.guard::<State<DashMap<String, AtomicUsize>>>().unwrap();
        if dash_map.get(&s).is_none() {
            dash_map.insert(s, AtomicUsize::new(1));
        } else {
            dash_map.get_mut(&s).unwrap().fetch_add(1, Ordering::SeqCst);
        }

        let mut a = ApiUser {
            name: String::from("api_user"),
        };

        println!("{:?}", dash_map);

        rocket::Outcome::Success(a)
/*        request.cookies()
            .get_private("api_user")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|val| ApiUser { name: val })
            .or_forward(())*/
    }
}
