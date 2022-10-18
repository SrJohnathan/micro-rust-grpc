mod test;

#[cfg(feature = "jcore")]
pub use jcore;



#[cfg(feature = "diesel")]
pub use diesel;










use jcore::async_trait::async_trait;






#[async_trait]
pub trait DAO<T,E,C>{
    async fn create(con: &C,value:E)-> Result<(bool,i32),String>;
    async fn update(con: &C,value:E)-> Result<bool,String>;
    async fn delete(con: &C,id:i32)-> Result<bool,String>;
    async fn get_all(con: &C) -> Result<Vec<T>,String>;
    async fn get_item(con: &C,id:i32)-> Result<T,String>;

}

#[async_trait]
pub trait LoginImpl<T,E,C>{
    async fn get(con: &C,value:E)-> Result<T,String>;
}










