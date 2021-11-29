#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
// use diesel::mysql::MysqlConnection;//prelude::*に入ってる？
use dotenv::dotenv;
use std::env;

use self::models::{Post, NewPost};

pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


// pub fn create_post<'a>(conn: &MysqlConnection, id: &'a i32, title: &'a str) -> usize {
//     use schema::posts;

//     let new_post = NewPost {
//         id: id,
//         title: title,
//     };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("Error saving new post")
// }