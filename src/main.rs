use actix_web::{App, HttpServer};
mod services;
use services::{create_user_article, fetch_user_articles, fetch_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://{}:{}", "127.0.0.1", 8080);

    HttpServer::new(move || {
        App::new()
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
