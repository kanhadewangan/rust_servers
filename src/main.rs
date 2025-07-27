use actix_web::{post, web::{self, Data, Json}, HttpResponse, Responder , HttpServer  , App};
use sqlx::PgPool;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
#[derive(Deserialize)]
struct Blogs {
    title: String,
    content: String,
}

#[post("/blog")]
async fn create_blog(blog: Json<Blogs>, db_pool: Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO blogs (title, content) VALUES ($1, $2)",
        blog.title,
        blog.content
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Blog created successfully"),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("DB insert failed")
        }
    }
}

#[actix_web::main]
async fn main () -> std::io::Result<()>{
dotenv().ok();
let db_urls = env::var("DATABASE_URL").expect("Seted");
let pool = PgPool::connect(&db_urls).await.expect("failed");

    HttpServer::new(
       move ||{
            App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_blog)
        }
    )
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
