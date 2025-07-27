#[allow(unused)]

use serde::Deserialize;
use actix_web::{cookie::time::Duration, get, post, web::{self, to, Json}, App, HttpResponse, HttpServer, Responder};
use std::{path, string, sync::Mutex};


#[derive(serde::Deserialize)]
struct Info{
    name:String,
    city:String
}
async fn index()->impl Responder{
    "hello Wrold"
}
struct AppState{
    app_name :String
}
// Josn
 #[derive(Deserialize)]
 struct PostData{
    id :String,
    name :String,
    city: String
 }
async fn post_req(path: web::Path<PostData>)-> impl Responder{
    format!(" id :{}  name: {}  city:{}", path.id , path.name , path.city )
}



async fn roots()->impl Responder{
    HttpResponse::Ok().body("Rooted")
}
 async  fn nest()-> impl Responder{
    HttpResponse::Ok().body("Nested")
 }


#[actix_web::main]

async fn main () -> std::io::Result<()>{
    
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(
        ||{
            App::new().service(
                web::scope("/rn")
                .route("/", web::get().to(roots))
                .route("/next", web::get().to(nest))
                
           
            )
            .route("/user/{id}/{name}/{city}", 
        web::post().to(post_req)
            )
         
            .app_data(web::Data::new(AppState {
                app_name:String::from("Hello")
            }))
            .route("/hello/{name}/{city}", web::get().to(greet))
            .service(data)
            .service(hello)
            .service(echo)
            .route("/", web::get().to(manual_hello))
            
        }
    )
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}


async fn greet(path: web::Path<Info>) -> impl Responder {
    format!("Hello, {} from {}!", path.name, path.city)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/name")]
async fn data(data:web::Data<AppState>)->String{
    let app_name = &data.app_name;
    format!("hii {app_name}")
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello()  -> impl Responder{
    HttpResponse::Ok().body("hello from Mmam")
}