use rocket::Request;
use rocket::request::FromRequest;
use rocket::outcome::IntoOutcome;

#[derive(Debug)]
pub struct AdminUser {
    pub name: String,
}

#[derive(Debug)]
pub enum AdminUserError {
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = AdminUserError;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("admin_user")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|val| AdminUser { name: val })
            .or_forward(())
    }
}
