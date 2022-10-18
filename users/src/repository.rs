use crate::db::{PgAsyncConnection, PoolPgAsyncConnection};
use crate::entity::db_models::{Auth, AuthLogin, AuthRes, NewAuth};
use diesel::{Connection, debug_query, PgConnection, QueryDsl};
use diesel::ExpressionMethods;
use diesel::r2d2::Pool;

use tokio_diesel::*;





pub struct Dao;

use super::schema::*;

use crate::jcore::async_trait;

pub use migre;

#[async_trait::async_trait]
impl  migre::DAO<Auth,NewAuth,PgAsyncConnection> for Dao {
    async fn create(con: &PgAsyncConnection, value: NewAuth) -> Result<(bool, i32), String> {
        todo!()
    }

    async fn update(con: &PgAsyncConnection, value: NewAuth) -> Result<bool, String> {
        todo!()
    }

    async fn delete(con: &PgAsyncConnection, id: i32) -> Result<bool, String> {
        let id_sql = auth::dsl::id.eq(id);
        let res = diesel::delete(auth::dsl::auth.filter(id_sql)).execute_async(con).await ;

        match res {
            Ok(x) => if x == 1 { Ok( true ) } else {  Ok(false ) },
            Err(e) => Err(e.to_string())
        }

    }

    async fn get_all(con: &PgAsyncConnection) -> Result<Vec<Auth>, String> {
        todo!()
    }

    async fn get_item(con: &PgAsyncConnection, id: i32) -> Result<Auth, String> {

        let id_sql = auth::dsl::id.eq(id);
        let value = auth::dsl::auth
            .filter( id_sql );

        // println!("{}", debug_query::<diesel::pg::Pg, _>(&value));
        let res = value .first_async( con ).await ;
        match  res {
            Ok(x) => {
                Ok(x)
            }
            Err(e) => {Err(e.to_string())}
        }

    }
}

#[async_trait::async_trait]
impl migre::LoginImpl<Auth,AuthLogin,PgAsyncConnection> for Dao {
    async fn get(con: &PgAsyncConnection, value: AuthLogin) -> Result<Auth, String> {

        let email = auth::dsl::email.eq(value.email);
        let pass =auth::dsl::password.eq(value.password);

        let value = auth::dsl::auth.
            filter(email)
            .filter( pass );

      // println!("{}", debug_query::<diesel::pg::Pg, _>(&value));

        let res = value .first_async( con ).await ;

        match  res {
            Ok(x) => {
                Ok(x)
            }
            Err(e) => {Err(e.to_string())}
        }
    }
}

