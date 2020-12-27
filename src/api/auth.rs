use actix_web::{dev, error, web, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;

use crate::State;

#[derive(Clone)]
pub struct BasicCreds {
    username: String,
    password: String,
}

impl BasicCreds {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn matches(&self, creds: BasicAuth) -> bool {
        creds.user_id() == &self.username
            && creds.password().is_some()
            && creds.password().unwrap() == &self.password
    }
}

pub async fn validator(
    req: dev::ServiceRequest,
    credentials: BasicAuth,
) -> Result<dev::ServiceRequest, Error> {
    let state = req
        .app_data::<web::Data<State>>()
        .expect("App data should be set");
    if state.creds.matches(credentials) {
        Ok(req)
    } else {
        Err(error::ErrorUnauthorized("Incorrect username or password"))
    }
}
