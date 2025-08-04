use std::iter::once;

use actix_web::{
    body::{BoxBody}, dev::{ServiceRequest, ServiceResponse}, http::header::ContentType, middleware::{from_fn, Next}, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error, get};
use serde::{Deserialize, Serialize};
use futures::{future::ok };
#[derive(Serialize, Deserialize)]

struct  MyStruct {
    title: String,
    content: String,
}

impl Responder for  MyStruct{
    type Body = BoxBody;

fn  respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    let body = serde_json::to_string(&self).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    }
}

async fn index() -> impl Responder {
    let my_struct = MyStruct {
        title: "Hello, World!".to_string(),
        content: "This is a simple Actix-web application.".to_string(),
    };
    my_struct
}
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Service is running")

}
#[get("/hello")]
async fn hello1() -> impl Responder {
    HttpResponse::Ok().json("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(from_fn(hello))
            .route("/", web::get().to(index))
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}




async fn hello<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse<B>, Error>
where
    B: actix_web::body::MessageBody + 'static,
{
    println!("hello");
    next.call(req).await
}


//#[cfg(test)]

// mod test{
//     use actix_web::web::head;
//     use actix_web::{test, App, http::header::ContentType};
//     use actix_web::test::init_service;

//     use super::*;
//     #[actix_web::test]
//     async fn test_index() {
//         let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp: MyStruct = test::call_and_read_body_json(&mut app, req).await;
//         assert_eq!(resp.title, "Hello, World!");
//         assert_eq!(resp.content, "This is a simple Actix-web application.");  


// }

// #[actix_web::test]
// async fn test_health() {
//     let mut app = test::init_service(App::new().route("/health", web::get().to(health_check))).await;
//     let req = test::TestRequest::get().uri("/health").to_request();
//     let resp: String = test::call_and_read_body_json(&mut app, req).await;
//     assert_eq!(resp, "Service is running");

// }

// }