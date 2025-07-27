
use serde::Deserialize;
use actix_web::{ delete, get, post, put, web::{self}, App, HttpResponse, HttpServer, Responder};


#[derive(serde::Deserialize)]
struct Info{
    name:String,
    city:String
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
            .service(create)
            .service(put)
            .service(delete)
            .service(health)
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

#[derive(Deserialize)]
#[allow(unused_variables)]
struct Blog{
    id:String,
    username:String,
    blogs:String
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
// Json data Parse
#[post("/blogs")]
async fn create(blog:web::Json<Blog>)->impl Responder{
    let blog = blog.into_inner();
    HttpResponse::Ok().json(format!("Recived blogs:  /n {} /n{}" , blog.id , blog.blogs))
}
#[put("/blogs")]
async fn put(blog:web::Json<Blog>)-> impl Responder{
    let blogs = blog.into_inner();
    format!("{} {} {}", blogs.id , blogs.username , blogs.blogs)
}
#[delete("/blog")]
async fn delete()-> impl Responder{
    HttpResponse::Ok().json("Delete SucessFully ")

}
#[derive(Deserialize)]
struct  Infom{
    name:String
}
// Query Params
#[get("/que")]
async  fn health(info:web::Query<Infom>)-> impl Responder{
    format!(" Hello from Query {}", info.name )
}
async fn manual_hello()  -> impl Responder{
    HttpResponse::Ok().body("hello from Mmam")
}