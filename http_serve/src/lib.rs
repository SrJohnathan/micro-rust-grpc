pub mod header;

use jsonwebtoken::errors::Error;
use jsonwebtoken::TokenData;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde_derive::{Deserialize, Serialize};

use crate::header::decode_token;
use rocket::http::Status;
use rocket::{routes, Request};

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SidToken {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SidToken {
    type Error = ();

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<SidToken, (Status, <Self as FromRequest<'r>>::Error), ()> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, ())),
            Some(x) => {
                let authen_str = x.to_string();
                if authen_str.starts_with("Bearer") {
                    let token = authen_str[6..authen_str.len()].trim();
                    match decode_token(token.to_string(), "KEY_SEQ".to_string()) {
                        Ok(x) => Outcome::Success(SidToken {
                            id: x.claims.user,
                            token: token.to_string(),
                        }),
                        Err(e) => Outcome::Failure((Status::Unauthorized, ())),
                    }
                }else { Outcome::Failure((Status::Unauthorized, ())) }
            }
        }
    }
}
