//#[macro_use]
extern crate tonic;
#[macro_use]
extern crate diesel;



use tonic::transport::Server;
use crate::entity::DAO;
use crate::entity::users::{NewUsuario, UsuarioDao};

// #[warn(unused_imports)]

mod fuctions;
mod entity;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "127.0.0.1:50051".parse().unwrap();
    let mut set = fuctions::MessageServer::default();
    //set.com = entity::db::connection().await?;

    println!("server listening on {}", addr);
    Server::builder()
        .add_service(fuctions::server_proto::message_type_server::MessageTypeServer::new(set))
        .serve(addr)
        .await?;
    Ok(())
}
