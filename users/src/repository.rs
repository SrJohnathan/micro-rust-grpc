use migre::AsyncConnection;
use crate::db::{ PgAsyncConnection};
use crate::entity::db_models::{Auth, AuthLogin, AuthRes, NewAuth};
use diesel::{insert_into, QueryDsl};
use diesel::ExpressionMethods;
use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};

struct Dao;

use super::schema::*;

use crate::jcore::async_trait;


#[async_trait::async_trait]
impl migre::LoginImpl<AuthRes,AuthLogin,AsyncConnection<PgAsyncConnection>> for Dao {
    async fn get(con: &AsyncConnection<AsyncConnection<PgAsyncConnection>>, value: AuthLogin) -> Result<AuthRes, String> {



        let email = auth::dsl::email.eq(value.email);
        let pass =auth::dsl::password.eq(value.password);

        match auth::dsl::auth.
            filter(email)
            .filter( pass )
        .first_async::<Auth>(&con.0.0).await{
            Ok(x) => {  Ok(x)}
            Err(e) => {Err(e.to_string())}
        }


    }
}

