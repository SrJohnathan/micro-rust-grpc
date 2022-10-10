
mod repository;

use users::migre::core::tokio::main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  /*  let addr = "127.0.0.1:50051".parse().unwrap();
    let mut set = fuctions::MessageServer::default();
    //set.com = entity::db::connection().await?;

    println!("server listening on {}", addr);
    Server::builder()
        .add_service(fuctions::server_proto::message_type_server::MessageTypeServer::new(set))
        .serve(addr)
        .await?; */
    Ok(())
}