use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use pokepoke_rust::*;
use models::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize}; //一旦Pokemonで使うので


use pokepoke_rust::schema::p_made_pokemons::dsl::*;
use pokepoke_rust::schema::p_base_pokemons::dsl::*;
use pokepoke_rust::schema::p_in_battle_pokemons::dsl::*;
use pokepoke_rust::schema::m_move_bases::dsl::*;
use pokepoke_rust::schema::s_players::dsl::*;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(do_login)
        .service(get_pokemon_and_relative))
        .bind("127.0.0.1:8081")?
        .run()
        .await
}


// #[post("/todos")]
// async fn post_todo(todo: web::Json<Todo>) -> impl Responder {
//     println!("post_todo");
//     println!("{:?}", todo);
//     HttpResponse::Ok().body("ok")
// }
//ログイン
#[post("/login")]
async fn do_login(data: web::Json<Player>) -> impl Responder {
    println!("{:?}", data);
    let connection = establish_connection();
    let searched_player = s_players.filter(pokepoke_rust::schema::s_players::player_id.eq(&data.player_id))
        .load::<Player>(&connection) //この時点でResult型(Ok(),Err())(Option型は(None, Some<T>)) 
        .unwrap_or_else(|_| vec!()); //これはErrかNoneのとき、引数のクロージャの戻り値を返し、Okのとき中身を返す（Result<T,E>,Some<T>のTと同じ型）
    println!("{:?}", searched_player);
    if searched_player == [] { //PlayerにEqを実装
        HttpResponse::BadRequest().body("ID is not found") //一旦BadRequestを使う
    } else if (&searched_player[0].password == &data.password) {
        HttpResponse::Ok().body("ok")
    } else {
        HttpResponse::BadRequest().body("Password is bad")
    }
}

#[get("/made-pokemon/{column_name}/{value}")]
async fn index(web::Path((column_name, value)): web::Path<(String, u32)>) -> impl Responder {
    println!("{},{}", column_name, value);
    let connection = establish_connection();
    let value: i32 = value as i32;
    let mut search_made_pokemon: Vec<madePokemon> = vec![Default::default()];

    match (&*column_name, value) {
        (_, 0) => search_made_pokemon = p_made_pokemons
                                        .load::<madePokemon>(&connection)
                                        .expect("Error loading posts"),
        ("player_id", _) => search_made_pokemon = p_made_pokemons.filter(pokepoke_rust::schema::p_made_pokemons::player_id.eq(value))
                                        .load::<madePokemon>(&connection)
                                        .expect("Error loading posts"),
        (_, _) => search_made_pokemon = p_made_pokemons.filter(pokepoke_rust::schema::p_made_pokemons::id.eq(value))
                                        .load::<madePokemon>(&connection)
                                        .expect("Error loading posts"),
    }    
    println!("{:?}", search_made_pokemon);
    web::Json(
        search_made_pokemon
    )
}

//ポケモン一匹の全情報
#[get("/pokemon-and-relative/{value}")]
async fn get_pokemon_and_relative(web::Path(value): web::Path<u32>) -> impl Responder {
    println!("{}", value);
    let connection = establish_connection();
    let value: i32 = value as i32;

    let search_made_pokemon = p_made_pokemons.filter(pokepoke_rust::schema::p_made_pokemons::id.eq(value))
        .load::<madePokemon>(&connection)
        .expect("Error loading p_made_pokemons");
    
    let search_base_pokemon = p_base_pokemons.filter(pokepoke_rust::schema::p_base_pokemons::id.eq(search_made_pokemon[0].base_pokemon_id))
        .load::<BasePokemon>(&connection)
        .expect("Error loading p_base_pokemons");
    
    let search_battle_pokemon = p_in_battle_pokemons.filter(made_pokemon_id.eq(value))
        .load::<InBattlePokemon>(&connection)
        .expect("Error loading p_in_battle_pokemons");

    let mut moves: Vec<Vec<MoveBase>> = vec![Default::default()];
    //技情報
    let search_move_base = |x:i32| m_move_bases.filter(pokepoke_rust::schema::m_move_bases::id.eq(x))
        .limit(5)
        .load::<MoveBase>(&connection)
        .expect("Error loading posts");
    moves.push(search_move_base(search_made_pokemon[0].move1_id));
    // moves.push(search_move_base(search_made_pokemon[0].move2_id));
    // moves.push(search_move_base(search_made_pokemon[0].move3_id));
    // moves.push(search_move_base(search_made_pokemon[0].move4_id));
    println!("{:?}", moves);

    web::Json(
        {ReturnPokemon 
            {made_pokemon: search_made_pokemon,
            base_pokemon: search_base_pokemon,
            battle_pokemon: search_battle_pokemon,
            moves: moves,
        }}
    )
}

//ポケモンの情報まとめる用
#[derive(Debug, Serialize)]
struct ReturnPokemon {
    made_pokemon: Vec<madePokemon>,
    base_pokemon: Vec<BasePokemon>,
    battle_pokemon: Vec<InBattlePokemon>,
    moves: Vec<Vec<MoveBase>>,
}
