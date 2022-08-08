use std::error::Error;
use std::fmt::{Debug, Formatter};
use diesel::{PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;


pub struct  MyAsyncConnection(pub(crate) Pool<ConnectionManager<PgConnection>>);

impl  Debug for MyAsyncConnection{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for MyAsyncConnection {
    fn default() -> Self {
        connection().unwrap()
    }
}

pub  fn connection() -> Result<MyAsyncConnection, Box<dyn Error>> {
    dotenv().unwrap();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let manager =
        ConnectionManager::<PgConnection>::new(db_url);
    Ok(MyAsyncConnection(Pool::builder().build(manager)?))
}