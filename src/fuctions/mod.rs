use std::fmt::{Debug, Formatter};
use std::mem::transmute;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use crate::fuctions::server_proto::{Message, Res};
use crate::{DAO, NewUsuario, UsuarioDao};
use crate::entity::db::MyAsyncConnection;


pub mod macros;


// macro enumeração dos enum
#[repr(i32)]
pub enum TypeMessageEnum {
    GetUsers = 1,
    GetUser,
    SetUser,
}


pub mod server_proto {
    // tonic::include_proto!("service_routes");
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        #[prost(int64, tag = "1")]
        pub id: i64,
        #[prost(string, tag = "2")]
        pub data: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub r#type: i32,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(bool, tag = "1")]
        pub res: bool,
        #[prost(string, tag = "2")]
        pub data: ::prost::alloc::string::String,
    }

    /// Generated client implementations.
    pub mod message_type_client {
        #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

        use tonic::codegen::*;

        #[derive(Debug, Clone)]
        pub struct MessageTypeClient<T> {
            inner: tonic::client::Grpc<T>,
        }

        impl MessageTypeClient<tonic::transport::Channel> {
            /// Attempt to create a new client by connecting to a given endpoint.
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
                where
                    D: std::convert::TryInto<tonic::transport::Endpoint>,
                    D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }

        impl<T> MessageTypeClient<T>
            where
                T: tonic::client::GrpcService<tonic::body::BoxBody>,
                T::Error: Into<StdError>,
                T::ResponseBody: Body<Data=Bytes> + Send + 'static,
                <T::ResponseBody as Body>::Error: Into<StdError> + Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> MessageTypeClient<InterceptedService<T, F>>
                where
                    F: tonic::service::Interceptor,
                    T::ResponseBody: Default,
                    T: tonic::codegen::Service<
                        http::Request<tonic::body::BoxBody>,
                        Response=http::Response<
                            <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                        >,
                    >,
                    <T as tonic::codegen::Service<
                        http::Request<tonic::body::BoxBody>,
                    >>::Error: Into<StdError> + Send + Sync,
            {
                MessageTypeClient::new(InterceptedService::new(inner, interceptor))
            }
            /// Compress requests with `gzip`.
            ///
            /// This requires the server to support it otherwise it might respond with an
            /// error.
            #[must_use]
            pub fn send_gzip(mut self) -> Self {
                self.inner = self.inner.send_gzip();
                self
            }
            /// Enable decompressing responses with `gzip`.
            #[must_use]
            pub fn accept_gzip(mut self) -> Self {
                self.inner = self.inner.accept_gzip();
                self
            }
            pub async fn send_message(
                &mut self,
                request: impl tonic::IntoRequest<super::Message>,
            ) -> Result<tonic::Response<super::Res>, tonic::Status> {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::new(
                            tonic::Code::Unknown,
                            format!("Service was not ready: {}", e.into()),
                        )
                    })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/service_routes.MessageType/SendMessage",
                );
                self.inner.unary(request.into_request(), path, codec).await
            }
        }
    }

    /// Generated server implementations.
    pub mod message_type_server {
        #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

        use tonic::codegen::*;

        ///Generated trait containing gRPC methods that should be implemented for use with MessageTypeServer.
        #[async_trait]
        pub trait MessageType: Send + Sync + 'static {
            async fn send_message(
                &self,
                request: tonic::Request<super::Message>,
            ) -> Result<tonic::Response<super::Res>, tonic::Status>;
        }

        #[derive(Debug)]
        pub struct MessageTypeServer<T: MessageType> {
            inner: _Inner<T>,
            accept_compression_encodings: (),
            send_compression_encodings: (),
        }

        struct _Inner<T>(Arc<T>);

        impl<T: MessageType> MessageTypeServer<T> {
            pub fn new(inner: T) -> Self {
                Self::from_arc(Arc::new(inner))
            }
            pub fn from_arc(inner: Arc<T>) -> Self {
                let inner = _Inner(inner);
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InterceptedService<Self, F>
                where
                    F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
        }

        impl<T, B> tonic::codegen::Service<http::Request<B>> for MessageTypeServer<T>
            where
                T: MessageType,
                B: Body + Send + 'static,
                B::Error: Into<StdError> + Send + 'static,
        {
            type Response = http::Response<tonic::body::BoxBody>;
            type Error = std::convert::Infallible;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(
                &mut self,
                _cx: &mut Context<'_>,
            ) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                let inner = self.inner.clone();
                match req.uri().path() {
                    "/service_routes.MessageType/SendMessage" => {
                        #[allow(non_camel_case_types)]
                        struct SendMessageSvc<T: MessageType>(pub Arc<T>);
                        impl<T: MessageType> tonic::server::UnaryService<super::Message>
                        for SendMessageSvc<T> {
                            type Response = super::Res;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::Message>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).send_message(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = SendMessageSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => {
                        Box::pin(async move {
                            Ok(
                                http::Response::builder()
                                    .status(200)
                                    .header("grpc-status", "12")
                                    .header("content-type", "application/grpc")
                                    .body(empty_body())
                                    .unwrap(),
                            )
                        })
                    }
                }
            }
        }

        impl<T: MessageType> Clone for MessageTypeServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                }
            }
        }

        impl<T: MessageType> Clone for _Inner<T> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }

        impl<T: MessageType> tonic::transport::NamedService for MessageTypeServer<T> {
            const NAME: &'static str = "service_routes.MessageType";
        }
    }
}

#[derive(Debug, Default)]
pub struct MessageServer {
    pub com: MyAsyncConnection,
}

impl MessageServer {
    fn get_connection_db_ref(&self) -> &MyAsyncConnection {
        &self.com
    }
}

#[tonic::async_trait]
impl server_proto::message_type_server::MessageType for MessageServer {
    async fn send_message(&self, request: Request<Message>) -> Result<Response<Res>, Status> {
        let message = request.into_inner();

        let en: TypeMessageEnum = unsafe { transmute(message.r#type) };

        match en {
            TypeMessageEnum::GetUsers => {
                let value = UsuarioDao::get_all(
                    self.get_connection_db_ref()
                ).await.unwrap();
                let str = serde_json::to_string(&value).unwrap();
                let res = Res { res: true, data: str };
                Ok(Response::new(res))
            }
            TypeMessageEnum::GetUser => {
                let value = UsuarioDao::get(
                    self.get_connection_db_ref(), message.id as i32,
                ).await.unwrap();
                let str = serde_json::to_string(&value).unwrap();
                let res = Res { res: true, data: str };
                Ok(Response::new(res))
            }
            TypeMessageEnum::SetUser => {
                let mut user: NewUsuario = serde_json::from_str(message.data.as_str()).unwrap();
                user.set_created_on();
                user.set_last_login();

                match UsuarioDao::create(
                    self.get_connection_db_ref(), user,
                ).await {
                    Ok(x) => {
                        let res = Res { res: true, data: x.1.to_string() };
                        Ok(Response::new(res))
                    }
                    Err(e) => {
                        let res = Res { res: false, data: e };
                        Ok(Response::new(res))
                    }
                }
            }


            //    println!("{:?}",request.into_inner().data);
        }
    }
}