use std::future::Future;
use std::path::Path;
#[macro_use]
use rocket;
use rocket::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use http_serve::header::BasicAuthRaw;
use users::db::{connection, PgAsyncConnection, Pool, PoolPgAsyncConnection};
use users::entity::db_models::AuthRes;
use jcore::tokio;

mod repository;

 pub struct Database<'a>(&'a PgAsyncConnection);
#[async_trait]
impl <'r>FromRequest<'r> for Database<'r> {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res =  request.guard::<&State<PgAsyncConnection>>().await.succeeded();
        match res {
            None => Outcome::Failure ((Status::ServiceUnavailable,())),
            Some(x) => Outcome::Success(Database(x.inner()))
        }



    }
}


#[rocket::main]
async fn main() -> Result<(),Error> {

  let con =   connection("DATABASE_URL_USERS".to_string() ).await.unwrap();
    let _ = rocket::build().manage(con)
        .mount("/", rocket::routes![
            repository::login,
            repository::get,
            repository::delete
        ]).launch().await.unwrap();

    Ok(())

  /*  let addr = "127.0.0.1:50051".parse().unwrap();
    let mut set = fuctions::MessageServer::default();
    //set.com = entity::db::connection().await?;

    println!("server listening on {}", addr);
    Server::builder()
        .add_service(fuctions::server_proto::message_type_server::MessageTypeServer::new(set))
        .serve(addr)
        .await?; */

}




