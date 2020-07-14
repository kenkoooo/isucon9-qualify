use actix_web::{web, App, HttpServer};
use isucari::server;
use isucari::IsuResult;
use sqlx::MySqlPool;
use std::env;

#[actix_rt::main]
async fn main() -> IsuResult<()> {
    let pool = MySqlPool::new(&format!(
        "mysql://{user}:{password}@{host}:{port}/{db}",
        host = env::var("MYSQL_HOST").unwrap_or("127.0.0.1".to_owned()),
        port = env::var("MYSQL_PORT").unwrap_or("3306".to_owned()),
        user = env::var("MYSQL_USER").unwrap_or("isucari".to_owned()),
        password = env::var("MYSQL_PASS").unwrap_or("isucari".to_owned()),
        db = env::var("MYSQL_DBNAME").unwrap_or("isucari".to_owned())
    ))
    .await?;
    HttpServer::new(move || {
        App::new()
            .service(server::post_initialize)
            .route("/login", web::get().to(server::get_index))
            .route("/register", web::get().to(server::get_index))
            .route("/timeline", web::get().to(server::get_index))
            .route("/categories/{c}/items", web::get().to(server::get_index))
            .route("/sell", web::get().to(server::get_index))
            .route("/items/{item_id}", web::get().to(server::get_index))
            .route("/items/{item_id}/edit", web::get().to(server::get_index))
            .route("/items/{item_id}/buy", web::get().to(server::get_index))
            .route("/buy/compelete", web::get().to(server::get_index))
            .route("/transactions/{t_id}", web::get().to(server::get_index))
            .route("/users/{user_id}", web::get().to(server::get_index))
            .route("/users/setting", web::get().to(server::get_index))
            .service(actix_files::Files::new("/", "../public/").show_files_listing())
            .service(actix_files::Files::new("/static", "../public/static/").show_files_listing())
            .data(pool.clone())
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;
    Ok(())
}
