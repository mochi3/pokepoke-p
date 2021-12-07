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
use pokepoke_rust::schema::s_rooms::dsl::*;
use pokepoke_rust::schema::s_selected_pokemons::dsl::*;

use diesel::associations::HasTable;

// use schema::s_rooms;
// use schema::s_selected_pokemons;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(do_login)
        // .service(get_pokemon_and_relative)
        .service(make_room)
        .service(post_selected_pokemon)
        .service(get_selected_pokemon)
        .service(post_battle_pokemon)
        .service(get_battle_pokemon)
        .service(do_command)
        )
        .bind("127.0.0.1:8081")?
        .run()
        .await
}


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
        HttpResponse::Ok().json(&searched_player[0])
    } else {
        HttpResponse::BadRequest().body("Password is bad")
    }
}

//①作ったポケモン一覧取得、
#[get("/made-pokemon/{column_name}/{value}")]
async fn index(web::Path((column_name, value)): web::Path<(String, u32)>) -> impl Responder {
    println!("{},{}", column_name, value);
    let connection = establish_connection();
    let value: i32 = value as i32;
    let mut searched_made_pokemon: Vec<MadePokemon> = vec![Default::default()];

    match (&*column_name, value) {
        (_, 0) => searched_made_pokemon = p_made_pokemons
                                        .load::<MadePokemon>(&connection)
                                        .expect("Error loading posts"),
        ("player_id", _) => searched_made_pokemon = p_made_pokemons.filter(pokepoke_rust::schema::p_made_pokemons::player_id.eq(value))
                                        .load::<MadePokemon>(&connection)
                                        .expect("Error loading posts"),
        (_, _) => searched_made_pokemon = search_made_pokemon(&connection, value),
    }    
    println!("{:?}", searched_made_pokemon);
    web::Json(
        searched_made_pokemon
    )
}

//部屋作成
#[post("/make-room")]
async fn make_room(data: String) -> impl Responder {
    let connection = establish_connection();
    let mut room: Room = serde_json::from_str(&data).unwrap();
    room.room_id = gen_random_str();

    insert_room(&connection, &room);

    let inserted_room = s_rooms.filter(pokepoke_rust::schema::s_rooms::room_id.eq(room.room_id))
        .load::<Room>(&connection)
        .expect("Error loading posts");

    HttpResponse::Ok().json(&inserted_room[0])
}

//見せ合い時のポケモン登録
#[post("/selected-pokemon")]
async fn post_selected_pokemon(data: String) -> impl Responder {
    let connection = establish_connection();
    let selected_pokemon: SelectedPokemon = serde_json::from_str(&data).unwrap();
    insert_selected_pokemon(&connection, &selected_pokemon);
    HttpResponse::Ok()
}

//見せ合い時のポケモン取得
#[get("/selected-pokemon/{req_room_id}/{req_player_id}")]
async fn get_selected_pokemon(web::Path((req_room_id, req_player_id)): web::Path<(i32, i32)>) -> impl Responder {
    let connection = establish_connection();
    let your_player_id = search_another_player(&connection, &req_room_id, &req_player_id);

    let my_selected_pokemons = &search_select_pokemon(&connection, req_player_id)[0];
    let your_selected_pokemons = &search_select_pokemon(&connection, your_player_id)[0];

    let my_made_pokemons = search_made_pokemons_by_selected_pokemons(&connection, my_selected_pokemons);
    let your_made_pokemons = search_made_pokemons_by_selected_pokemons(&connection, your_selected_pokemons);

    HttpResponse::Ok().json([my_made_pokemons, your_made_pokemons])
}

//バトル開始時ポケモン登録
#[post("/battle-pokemon")]
async fn post_battle_pokemon(data: String) -> impl Responder {
    println!("{:?}", data);
    let connection = establish_connection();
    let ids: Vec<i32> = serde_json::from_str(&data).unwrap();
    let length = ids.len();
    let mut battle_pokemons :Vec<BattlePokemon> = vec![Default::default(); length];
    for (i, v) in ids.iter().enumerate() {
        battle_pokemons[i].made_pokemon_id = *v;
        battle_pokemons[i].number = i as i32 +1;
    }
    println!("{:?}", battle_pokemons);
    for v in battle_pokemons.iter() {
        insert_battle_pokemon(&connection, &v);
    }
    //フィールド情報にも登録しておく
    HttpResponse::Ok()
}

//バトル開始時ポケモン取得
#[get("/battle-pokemon/{req_room_id}/{req_player_id}")]
async fn get_battle_pokemon(web::Path((req_room_id, req_player_id)): web::Path<(i32, i32)>) -> impl Responder {
    let connection = establish_connection();
    let your_player_id = search_another_player(&connection, &req_room_id, &req_player_id);

    let my_battle_pokemons: Vec<BattlePokemon> = search_battle_pokemons(&connection, req_player_id, "player_id");
    let your_battle_pokemons: Vec<BattlePokemon> = search_battle_pokemons(&connection, your_player_id, "player_id");
    // let my_first_id: &i32 = &my_battle_pokemons.into_iter().filter(|v| v.number == 1).collect::<Vec<BattlePokemon>>()[0].made_pokemon_id;
    let my_first_id: i32 = search_battle_pokemons_first(&connection, req_player_id)[0].made_pokemon_id;
    let your_first_id: i32 = search_battle_pokemons_first(&connection, your_player_id)[0].made_pokemon_id;
    let my_first_pokemon = return_pokemon(&connection, my_first_id);
    let your_first_pokemon = return_pokemon(&connection, your_first_id);

    HttpResponse::Ok().json((my_first_pokemon, your_first_pokemon))
}


//バトルコマンド
#[post("/do-command/{req_room_id}/{req_player_id}")]
async fn do_command(web::Path((req_room_id, req_player_id)): web::Path<(i32, i32)>, data: String) -> impl Responder {
    let connection = establish_connection();
    //コマンドをDBに登録
    let turn = 1;
    let mut my_command: Command = serde_json::from_str(&data).unwrap();
    my_command.turn = turn;
    println!("{:?}", my_command);
    insert_command(&connection, &my_command);

    //相手の情報取得
    let your_player_id = search_another_player(&connection, &req_room_id, &req_player_id);
    let your_command = &search_command(&connection, req_room_id, your_player_id, turn)[0];
    println!("{:?}", your_command);

    //それぞれのポケモン、技
    let my_pokemon_id = search_field_pokemon(&connection, req_room_id, req_player_id);
    let your_pokemon_id = search_field_pokemon(&connection, req_room_id, your_player_id);
    let my_pokemon = return_pokemon(&connection, my_pokemon_id);
    let your_pokemon = return_pokemon(&connection, your_pokemon_id);
    println!("{:?},{:?}", my_pokemon, your_pokemon);

    //両方がバトル選択

    //技検索
    let my_move = search_move_base(&connection, my_command.command_id);
    let your_move = search_move_base(&connection, your_command.command_id);
    let my_battle_info = BattleInfo::new(req_player_id, my_pokemon, my_move);
    let your_battle_info = BattleInfo::new(your_player_id, your_pokemon, your_move);

    //素早さ比較
    let (first, second) = if check_faster(&my_battle_info.pokemon, &your_battle_info.pokemon) {(my_battle_info, your_battle_info)} else {(your_battle_info, my_battle_info)};
    println!("{:?}", first);

    //フロントに返すやつ
    let mut show_battles: Vec<ShowBattle> = vec![Default::default()];
    show_battles = doBattle(&first, &second, show_battles);
    show_battles = doBattle(&second, &first, show_battles);
    println!("{:?}", show_battles);

    HttpResponse::Ok().json(show_battles)
}


//ポケモン一匹の全情報
// #[get("/pokemon-and-relative/{value}")]
// async fn get_pokemon_and_relative(web::Path(value): web::Path<u32>) -> impl Responder {
//     println!("{}", value);
//     let connection = establish_connection();
//     let value: i32 = value as i32;

//     let search_made_pokemon = search_made_pokemon(&connection, value);
//     let search_base_pokemon = search_base_pokemon(&connection, search_made_pokemon[0].base_pokemon_id);
    
//     let search_battle_pokemon = p_in_battle_pokemons.filter(made_pokemon_id.eq(value))
//         .load::<BattlePokemon>(&connection)
//         .expect("Error loading p_in_battle_pokemons");

//     let mut moves: Vec<Vec<MoveBase>> = vec![Default::default()];
//     //技情報
//     moves.push(search_move_base(&connection, search_made_pokemon[0].move1_id));
//     // moves.push(search_move_base(&connection, search_made_pokemon[0].move2_id));
//     // moves.push(search_move_base(&connection, search_made_pokemon[0].move3_id));
//     // moves.push(search_move_base(&connection, search_made_pokemon[0].move4_id));
//     println!("{:?}", moves);

//     web::Json(
//         {ReturnPokemon 
//             {made_pokemon: search_made_pokemon,
//             base_pokemon: search_base_pokemon,
//             battle_pokemon: search_battle_pokemon,
//             moves: moves,
//         }}
//     )
// }

