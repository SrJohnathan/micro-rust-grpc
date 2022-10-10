use chrono::Utc;
use diesel::{insert_into, QueryDsl};

use serde_derive::{Serialize, Deserialize};
use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::entity::DAO;
use migre::db::MyAsyncConnection;
use crate::entity::schema::usuario;


#[derive(Queryable, Debug,Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub numero: Option<String>,
    pub serie: String,
    pub email: String,
    pub id_firebase: String,
    pub token_firebase: Option<String>,
    pub created_on: Option<String>,
    pub last_login: Option<String>,
}

#[derive(Insertable, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "usuario"]
pub struct NewUsuario {
    pub name: String,
    pub password: String,
    pub numero: Option<String>,
    pub serie: String,
    pub email: String,
    pub id_firebase: String,
    pub token_firebase: Option<String>,
    pub created_on: Option<String>,
    pub last_login: Option<String>,
}


impl NewUsuario {
    pub fn new(name: String, password: String, numero: Option<String>,serie: String, email: String, id_firebase: String, token_firebase: Option<String>, created_on: Option<String>, last_login: Option<String>) -> Self {
        Self { name, password, numero, serie,email, id_firebase, token_firebase, created_on, last_login }
    }

    pub fn set_created_on(&mut self) {
        self.created_on = Some(Utc::now().naive_utc().to_string())
    }
    pub fn set_last_login(&mut self) {
        self.last_login = Some(Utc::now().naive_utc().to_string())
    }
}


pub struct UsuarioDao {}

#[async_trait::async_trait]
impl DAO<Usuario, NewUsuario> for UsuarioDao {
    async fn create(con: &MyAsyncConnection, value: NewUsuario) -> Result<(bool, i32), String> {
        match insert_into(usuario::table)
            .values(&value)
            .returning(usuario::id)
            .get_result_async::<i32>(&con.0).await{
            Ok(x) => {  Ok((true, x))}
            Err(e) => {Err(e.to_string())}
        }

    }

    async fn update(con: &MyAsyncConnection, value: NewUsuario) -> Result<bool, String> {

    }

    async fn delete(con: &MyAsyncConnection, id: i32) -> Result<bool, String> {
     match  diesel::delete(usuario::dsl::usuario.filter(usuario::dsl::id.eq(1))).execute_async(&con.0).await{
         Ok(x) => {  Ok( if(x  as i32) > 0 { true } else {  false }   )}
         Err(e) => {Err(e.to_string())}
     }

    }

    async fn get_all(con: &MyAsyncConnection) -> Result<Vec<Usuario>, String> {
        match usuario::dsl::usuario.load_async::<Usuario>(&con.0)
           .await{
            Ok(x) => {  Ok(x)}
            Err(e) => {Err(e.to_string())}
        }
    }

    async fn get(con: &MyAsyncConnection, id: i32) -> Result<Usuario, String> {
      match  usuario::dsl::usuario.filter(usuario::dsl::id.eq(id)).first_async::<Usuario>(&con.0).await{
          Ok(x) => {  Ok(x)}
          Err(e) => {Err(e.to_string())}
        }

    }
}
