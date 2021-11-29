use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, ResponseError, Error};
use thiserror::Error;
use askama::Template;
use serde::Deserialize;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
// use std::error::Error;

use pokepoke_rust::models::NewPost;

// pub mod schema;
// pub mod models;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error), //askamaのエラーをMyErrorに変換している
    #[error("Failed to render HTMLj")]
    ConnectionPoolError(#[from] r2d2::Error), //askamaのエラーをMyErrorに変換している
}

impl ResponseError for MyError {}

// #[add("/add")]
// async fn add_todo(
//     params: web::Form<AddParams>,
//     db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
// ) -> Result<HttpResponse, MyError> {
//     let conn = db.get()?;
//     conn.execute();
//     Ok(HttpResponse::SeeOther()
//         .header(header::LOCATION, "/")
//         .finish())
// }

// Handler for POST /users
// #[post("/add")]
// pub async fn add_user(
//     db: web::Data<Pool>,
//     item: web::Json<NewPost>,
// ) -> Result<HttpResponse, dyn Error> {
//     Ok(web::block(move || create_post(db, item))
//         .await
//         .map(|post| HttpResponse::Created().json(post))
//         .map_err(|_| HttpResponse::InternalServerError())?)
// }

// fn add_single_user(
//     db: web::Data<Pool>,
//     item: web::Json<InputUser>,
// ) -> Result<User, diesel::result::Error> {
//     let conn = db.get().unwrap();
//     let new_user = NewUser {
//         first_name: &item.first_name,
//         last_name: &item.last_name,
//         email: &item.email,
//         created_at: chrono::Local::now().naive_local(),
//     };
//     let res = insert_into(users).values(&new_user).get_result(&conn)?;
//     Ok(res)
// }

// pub fn create_post<'a>(
//     db: web::Data<Pool>,
//     item: web::Json<NewPost>,
// ) -> Result<NewPost, diesel::result::Error> {
//     let conn = db.get().unwrap();
//     use pokepoke_rust::schema::posts;

//     let new_post = NewPost {
//         id: item.id,
//         title: item.title,
//     };

//     let res = diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("Error saving new post");

//     Ok(res)
// }

// #[post("/add")]
// async fn add_todo(
//     params: web::Form<NewPost>,
//     db: web::Data<Pool>,
// ) -> Result<HttpResponse, MyError> {
//     let conn = db.get()?;
//     conn.execute();
//     Ok(HttpResponse::SeeOther()
//         .header(header::LOCATION, "/")
//         .finish())
// }



#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "first".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "second".to_string(),
    });
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
