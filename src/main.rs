
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
async fn index()->impl Responder{
    "hello Wrold"
}
struct AppState{
    app_name :String
}
struct AppStateCounter {
    counter: Mutex<i32>
}

async fn count(counts: web::Data<AppStateCounter>) -> String {
    let mut counter =  counts.counter.lock().unwrap();
    *counter+=1;
    format!("{counter}")

}

#[actix_web::main]

async fn main () -> std::io::Result<()>{
    
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(
        ||{
            App::new().service(
                web::scope("/app")
            .route("/index.html", web::get().to(index))
           
            )
            .app_data(web::Data::new(AppState {
                app_name:String::from("Hello")
            }))
            .app_data(count.clone())
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