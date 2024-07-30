use crate::db_utils::{AppState, DbActor};
use crate::messages::{CreateArticle, FetchUser, FetchUserArticles};
use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/user/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserArticles { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
    }
}

#[post("/user/{id}/article")]
pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>,
) -> impl Responder {
    let id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(CreateArticle {
            title: body.title.to_string(),
            content: body.content.to_string(),
            created_by: id,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create article"),
    }
}
