#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
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


use schema::posts;

// pub fn create_post<'a>(conn: &MysqlConnection, id: &'a i32, title: &'a str) -> usize {
//     // use schema::posts;

//     let new_post = NewPost {
//         id: *id,
//         title: title.to_string(),
//     };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("Error saving new post")
// }

// fn create<T>(conn: &MysqlConnection, new_post: T) -> usize {
//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("Error saving new post")
// }

// pub fn gen_random_str() -> String {
//     use rand::prelude::*;
//     let mut rng = rand::thread_rng();
//     let mut v: Vec<u8> = ('a' as u8..'z' as u8).collect();
//     v.shuffle(&mut rng);
//     return String::from_utf8(v).unwrap();
// }

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn gen_random_str() -> String {
    use rand::prelude::SliceRandom;
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, 10)
            .cloned()
            .collect()
    ).unwrap()
}


//APIの細かい処理
use models::*;
use schema::p_made_pokemons::dsl::*;
use schema::p_base_pokemons::dsl::*;
use schema::p_in_battle_pokemons::dsl::*;
use schema::m_move_bases::dsl::*;
use schema::s_players::dsl::*;
use schema::s_rooms::dsl::*;
use schema::s_selected_pokemons::dsl::*;

//-----------select---------------------------------

pub fn search_made_pokemon (connection: &MysqlConnection, value: i32)  -> Vec<MadePokemon> {
    p_made_pokemons.filter(schema::p_made_pokemons::id.eq(value))
    .load::<MadePokemon>(connection)
    .expect("Error loading posts")
}

pub fn search_base_pokemon (connection: &MysqlConnection, value: i32)  -> Vec<BasePokemon> {
    p_base_pokemons.filter(schema::p_base_pokemons::id.eq(value))
    .load::<BasePokemon>(connection)
    .expect("Error loading posts")
}

// pub fn search_battle_pokemon (connection: &MysqlConnection, value: i32)  -> Vec<MadePokemon> {
//     p_made_pokemons.filter(schema::p_made_pokemons::id.eq(value))
//     .load::<MadePokemon>(connection)
//     .expect("Error loading posts")
// }

pub fn search_room (connection: &MysqlConnection, value: i32) -> Vec<Room> {
    s_rooms.filter(schema::s_rooms::id.eq(value))
        .load::<Room>(connection)
        .expect("Error loading posts")
}

pub fn search_select_pokemon (connection: &MysqlConnection, value: i32) -> Vec<SelectedPokemon> {
    s_selected_pokemons.filter(schema::s_selected_pokemons::id.eq(value))
        .load::<SelectedPokemon>(connection)
        .expect("Error loading posts")
}

pub fn search_made_pokemons_by_selected_pokemons (connection: &MysqlConnection, pokemons: &SelectedPokemon) -> Vec<MadePokemon> {
    p_made_pokemons
        .filter(schema::p_made_pokemons::id.eq(pokemons.pokemon1_id))
        .or_filter(schema::p_made_pokemons::id.eq(pokemons.pokemon2_id))
        .or_filter(schema::p_made_pokemons::id.eq(pokemons.pokemon3_id))
        .or_filter(schema::p_made_pokemons::id.eq(pokemons.pokemon4_id))
        .or_filter(schema::p_made_pokemons::id.eq(pokemons.pokemon5_id))
        .or_filter(schema::p_made_pokemons::id.eq(pokemons.pokemon6_id))
        .load::<MadePokemon>(connection)
        .expect("Error loading p_made_pokemons")
}

pub fn search_move_base (connection: &MysqlConnection, value: i32) -> Vec<MoveBase> {
    m_move_bases.filter(schema::m_move_bases::id.eq(value))
        .load::<MoveBase>(connection)
        .expect("Error loading posts")
}

//-----------------insert-------------------------------------

pub fn insert_room (connection: &MysqlConnection, value: &Room) {
    diesel::insert_into(schema::s_rooms::table)
        .values(value)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn insert_selected_pokemon (connection: &MysqlConnection, value: &SelectedPokemon) {
    diesel::insert_into(schema::s_selected_pokemons::table)
        .values(value)
        .execute(connection)
        .expect("Error saving new post");
}