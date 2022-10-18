use std::ops::Deref;
use crate::Database;
use http_serve::header::{generate_token, BasicAuthRaw};
use rocket::response::status;
use rocket::response::status::{Accepted, Created, Custom, NotFound};
use rocket::serde::json::Json;
use rocket::*;
use serde_json::json;
use http_serve::SidToken;
use users::db::connection;
use users::entity::db_models::{Auth, AuthLogin, AuthRes, AuthResponse};
use users::repository::Dao;
use users::repository::migre::{DAO,LoginImpl};

#[post("/login")]
pub async fn login(
    _auth: BasicAuthRaw,
    _database:Database<'_>
) -> Result<Created<Json<AuthRes>>, NotFound<String>> {
    match Dao::get(
        _database.0,
        AuthLogin {
            email: _auth.username.to_string(),
            password: _auth.password.to_string(),
        },
    )
    .await
    {
        Ok(x) => {
            let token = generate_token(x.id, "KEY_SEQ".to_string());
            Ok(Created::new("").body(Json(AuthRes {
                id: x.id,
                email: x.email,
                token,
            })))
        }
        Err(x) => Err(NotFound(x)),
    }
}


#[get("/user/<id>")]
pub async fn get(
    id:i32,
    _auth: SidToken,
    _database:Database<'_>
) -> Result<Created<Json<AuthResponse>>, NotFound<String>> {
    match Dao::get_item(
        _database.0,
        id
    )
        .await
    {
        Ok(x) => {
            Ok(Created::new("").body(Json(AuthResponse {
                id: x.id,
                email: x.email,
                serie:x.serie,
                numero : x.numero,
                name : x.name,
                token_firebase : x.token_firebase,
                id_firebase: x.id_firebase

            })))
        }
        Err(x) => Err(NotFound(x)),
    }
}

#[delete("/user/<id>")]
pub async fn delete(
    id:i32,
    _auth: SidToken,
    _database:Database<'_>
) -> Result<Custom<String>, NotFound<String>> {
    match Dao::delete(
        _database.0,
        id
    )
        .await
    {
        Ok(x) => {
            Ok(status::Custom(http::Status::Ok, i32::from(x).to_string()))
        }
        Err(x) => Err(NotFound(x)),
    }
}