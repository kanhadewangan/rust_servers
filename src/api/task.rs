 
//  use actix_web::{
//     get,
//     post,
//     put,
//     error::ResponseError,
//     web::Path,
//     web::Json,
//     web::Data,
//     HttpResponse,
//     http::{header::ContentType, StatusCode}
//  };
//  use serde::{Serialize, Deserialize};
//  use derive_more::{Display};

//  #[derive(Deserialize, Serialize)]
// pub struct Task{
//     task_id :String,

// }

//  #[get("/task{task_global_id}")]
//  pub async fn get_task()->Json<String>{
//      return Json("hello Worls".to_string());
//  }
#[actix_web::main]
async fn main () -> std::io::Result<()>{
dotenv.ok();
let db_urls = env::var("DATABASE_URL").expect("Seted");
let pool = PgPool::connect(&db_urls).await.expect("failed");

    HttpServer::new(
       move ||{
            App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/blog",web::post().to(create_blog))
        }
    )
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}