use actix_web::{App, HttpServer};
use sqlx::sqlite::SqlitePoolOptions;

mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://ethbank.db")
        .await
        .expect("Erro ao conectar ao banco de dados");

    db::init_db(&pool).await;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
