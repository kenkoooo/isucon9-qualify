use actix_web::{App, HttpServer};
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
            .service(server::initialize::initialize)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;
    Ok(())
}
