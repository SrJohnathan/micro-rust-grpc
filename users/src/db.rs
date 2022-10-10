use std::error::Error;
use std::fmt::{Debug, Formatter};
use diesel::{PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::embed_migrations;
use migre::jcore::dotenv::dotenv;


pub struct PgAsyncConnection(pub Pool<ConnectionManager<PgConnection>>);

impl Debug for PgAsyncConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for PgAsyncConnection {
    fn default() -> Self {
        connection("GRPC".to_string()).unwrap()
    }
}

pub fn connection(str: String) -> Result<PgAsyncConnection, Box<dyn Error>> {
    dotenv().unwrap();
    let db_url = std::env::var(str).unwrap();
    let manager =
        ConnectionManager::<PgConnection>::new(db_url);

   // embed_migrations!();

    Ok(PgAsyncConnection(Pool::builder().build(manager)?))
}