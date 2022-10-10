mod test;

#[cfg(feature = "jcore")]
pub use jcore;



#[cfg(feature = "diesel")]
pub use diesel;










use jcore::async_trait::async_trait;

pub struct AsyncConnection<T>(pub T);




#[async_trait]
pub trait DAO<T,E,C>{
    async fn create(con: &AsyncConnection<C>,value:E)-> Result<(bool,i32),String>;
    async fn update(con: &AsyncConnection<C>,value:E)-> Result<bool,String>;
    async fn delete(con: &AsyncConnection<C>,id:i32)-> Result<bool,String>;
    async fn get_all(con: &AsyncConnection<C>) -> Result<Vec<T>,String>;
    async fn get(con: &AsyncConnection<C>,id:i32)-> Result<T,String>;

}

#[async_trait]
pub trait LoginImpl<T,E,C>{
    async fn get(con: &AsyncConnection<C>,value:E)-> Result<T,String>;
}










