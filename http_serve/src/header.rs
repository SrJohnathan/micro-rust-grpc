use rocket::{Request, outcome::Outcome, http::Status, request::FromRequest, State, Response};
use serde_derive::{Serialize, Deserialize};

use reqwest::Client;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::response::{Responder, status};
use jcore::chrono::Utc;
use jcore::dotenv::dotenv;
use crate::UserToken;

#[derive(Serialize, Deserialize,Debug)]
pub struct BasicAuthRaw {
    pub username: String,
    pub password: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuthRaw {
    type  Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<BasicAuthRaw, (Status, <Self as FromRequest<'r>>::Error), ()> {



        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            let split = auth_header.split_whitespace().collect::<Vec<_>>();
            if split.len() != 2 {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
            let (basic, payload) = (split[0], split[1]);
            if basic != "Basic" {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
            let decoded = base64::decode(payload).unwrap();


            let decoded_str = String::from_utf8(decoded).unwrap();


            let split = decoded_str.split(":").collect::<Vec<_>>();


            if split.len() != 2 {
                return Outcome::Failure((Status::BadRequest, ()));
            }

            let (username, password) = (split[0].to_string(), split[1].to_string());

            let value = BasicAuthRaw {
                username,
                password,
            };
            Outcome::Success(value)
        } else {

             Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

pub fn generate_token(id:i32,key:String) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + (60 * 60 * 7),
        user: format!( "{}",id),
    };

    dotenv().unwrap();
    let secret_key = std::env::var(key).unwrap();
    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(secret_key.as_ref())).unwrap()
}

pub fn decode_token(token: String,key:String) -> Result<TokenData<UserToken>,jsonwebtoken::errors::Error>
{
    dotenv().unwrap();
    let secret_key = std::env::var(key).unwrap();
    let mut v = Validation::default();
    v.validate_exp = true;
    jsonwebtoken::decode::<UserToken>(&token.as_str(), &DecodingKey::from_secret(secret_key.as_ref()), &v)
}

