use std::error::Error;
use std::path::Path;
use migre::{ LoginImpl};
use users::{db, repository};
use users::db::PgAsyncConnection;
use users::entity::db_models::{Auth, AuthLogin};

#[tokio::test(flavor = "multi_thread")]
async fn test_db_ops() -> Result<(), Box<dyn Error>> {

    match   db::connection("DATABASE_URL".to_string()).await {
        Ok(x) => {

            let login = AuthLogin{ email : "12".to_string(), password : "123".to_string()};

          let value =  repository::Dao::get(&x,login).await;

            match value {
                Ok(auth) =>  println!("{:?}",auth),
                Err(e) =>  println!("{:?}",e)
            }



        }
        Err(error) => {}
    }


    Ok(())
}
