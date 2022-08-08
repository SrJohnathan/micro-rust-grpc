use crate::entity::db::MyAsyncConnection;

pub mod db;
pub mod users;
pub mod schema;


#[async_trait::async_trait]
pub trait DAO<T,E>{
   async fn create(con: &MyAsyncConnection,value:E)-> Result<(bool,i32),String>;
    async fn update(con: &MyAsyncConnection,value:E)-> Result<bool,String>;
    async fn delete(con: &MyAsyncConnection,id:i32)-> Result<bool,String>;
    async fn get_all(con: &MyAsyncConnection) -> Result<Vec<T>,String>;
    async fn get(con: &MyAsyncConnection,id:i32)-> Result<T,String>;

}