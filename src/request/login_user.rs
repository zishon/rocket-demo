use rocket::Request;
use rocket::request::FromRequest;
use rocket::outcome::IntoOutcome;

#[derive(Debug)]
pub struct User {
    pub name: String,
}

#[derive(Debug)]
pub enum UserError {
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = UserError;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("login_user")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|val| User { name: val })
            .or_forward(())
    }
}
