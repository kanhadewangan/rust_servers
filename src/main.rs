
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
async fn index()->impl Responder{
    "hello Wrold"
}
struct AppState{
    app_name :String
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
         
            .app_data(web::Data::new(AppState {
                app_name:String::from("Hello")
            }))
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