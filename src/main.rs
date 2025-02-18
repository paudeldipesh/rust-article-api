use actix::{Addr, SyncArbiter};
use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;
mod actors;
mod db_models;
mod db_utils;
mod insertables;
mod messages;
mod schema;
mod services;
use crate::db_utils::{get_pool, AppState, DbActor};
use crate::services::{create_user_article, fetch_user_articles, fetch_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("Database URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr: Addr<DbActor> = SyncArbiter::start(5, move || DbActor(pool.clone()));

    println!("Server running at http://{}:{}", "127.0.0.1", 8080);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
