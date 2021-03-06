use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use pokepoke_rust::*;
use models::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize}; //一旦Pokemonで使うので
use std::time::Duration;


use pokepoke_rust::schema::p_made_pokemons::dsl::*;
use pokepoke_rust::schema::p_base_pokemons::dsl::*;
use pokepoke_rust::schema::p_in_battle_pokemons::dsl::*;
use pokepoke_rust::schema::m_move_bases::dsl::*;
use pokepoke_rust::schema::s_players::dsl::*;
use pokepoke_rust::schema::s_rooms::dsl::*;
use pokepoke_rust::schema::s_selected_pokemons::dsl::*;

// use diesel::associations::HasTable;

// use schema::s_rooms;
// use schema::s_selected_pokemons;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(do_login)
        .service(get_make_pokemon)
        .service(post_make_pokemon)
        // .service(get_pokemon_and_relative)
        .service(make_room)
        .service(post_selected_pokemon)
        .service(get_selected_pokemon)
        .service(post_battle_pokemon)
        .service(get_battle_pokemon)
        .service(get_change_pokemons)
        .service(do_command)
        .service(print)
        )
        .bind("127.0.0.1:8081")?
        .run()
        .await //Future に対して .await を作用させると、Future の完了を待って、その結果を取得することができます
}


#[get("/print/{_player_id}")]
async fn print(web::Path(_player_id): web::Path<u32>) -> impl Responder { //asyncがついてるので非同期関数（futureを返す関数）
    for i in 0..10 {
        // re(i).await;
        println!("{}", 1);
        tokio::time::delay_for(Duration::from_millis(1000)).await; //n秒待つ
        if i > 5 {
            println!("{}", "こえた");
            break;
        }
    }
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // let task = async {
    //     for i in 0..10 {
    //         println!("{}", _player_id);
    //         tokio::time::delay_for(Duration::from_millis(1000)).await; //n秒待つ
    //         if i > 5 {
    //             println!("{}", "こえた");
    //         };
    //     };
    // };
    // rt.block_on(task);
    println!("{}", "done");
    HttpResponse::Ok()
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
    } else if &searched_player[0].password == &data.password {
        HttpResponse::Ok().json(&searched_player[0])
    } else {
        HttpResponse::BadRequest().body("Password is bad")
    }
}

//ポケモン作成用のポケモン・わざ情報一覧取得
#[get("/make-pokemon")]
async fn get_make_pokemon() -> impl Responder {
    let connection = establish_connection();
    let base_pokemons = search_base_pokemons(&connection, None);
    let move_bases = search_move_bases(&connection, None);

    HttpResponse::Ok().json((base_pokemons, move_bases))
}

//ポケモン作成
#[post("/make-pokemon")]
async fn post_make_pokemon(data: String) -> impl Responder {
    let connection = establish_connection();
    let pokemon: MadePokemon = serde_json::from_str(&data).unwrap();
    insert_made_pokemon(&connection, &pokemon);

    HttpResponse::Ok()
}

//作ったポケモン一覧取得
#[get("/made-pokemon/{req_player_id}")]
async fn index(web::Path(req_player_id): web::Path<i32>) -> impl Responder {
    let connection = establish_connection();
    web::Json(
        search_made_pokemons(&connection, None, Some(req_player_id))
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

    let my_selected_pokemons = search_select_pokemons(&connection, Some(req_player_id)).pop().unwrap();
    let your_selected_pokemons = search_select_pokemons(&connection, Some(your_player_id)).pop().unwrap();

    let my_made_pokemons = search_made_pokemonss_by_selected_pokemons(&connection, my_selected_pokemons);
    let your_made_pokemons = search_made_pokemonss_by_selected_pokemons(&connection, your_selected_pokemons);

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

    let my_first_id: i32 = search_battle_pokemons(&connection, None, Some(req_player_id), Some(1)).pop().unwrap().made_pokemon_id;
    let your_first_id: i32 = search_battle_pokemons(&connection, None, Some(your_player_id), Some(1)).pop().unwrap().made_pokemon_id;
    let my_first_pokemon = return_pokemon(&connection, my_first_id);
    let your_first_pokemon = return_pokemon(&connection, your_first_id);

    HttpResponse::Ok().json((my_first_pokemon, your_first_pokemon))
}

//バトル時ポケモン交換用ポケモン一覧取得
#[get("/battle-change-pokemons/{req_player_id}")] //room_idもつけた方がいい？
async fn get_change_pokemons(web::Path(req_player_id): web::Path<i32>) -> impl Responder {
    let connection = establish_connection();
    let mut battle_made_pokemons: Vec<_> = Vec::new();
    let my_battle_pokemons: Vec<BattlePokemon> = search_battle_pokemons(&connection, None, Some(req_player_id), None);
    for battle_pokemon in my_battle_pokemons.into_iter() {
        battle_made_pokemons.push((battle_pokemon, search_made_pokemons(&connection, Some(battle_pokemon.made_pokemon_id), None).pop()));
    }
    HttpResponse::Ok().json(battle_made_pokemons)
}

//バトルコマンド
#[post("/do-command/{req_room_id}/{req_player_id}")]
async fn do_command(web::Path((req_room_id, req_player_id)): web::Path<(i32, i32)>, data: String) -> impl Responder {
    let connection = establish_connection();
    //現在ターン取得
    let turn = search_fields(&connection, Some(req_room_id)).pop().unwrap().turn;
    
    //コマンドをDBに登録
    let mut my_command: Command = serde_json::from_str(&data).unwrap();
    my_command.turn = turn;
    insert_command(&connection, &my_command);

    //相手の情報取得
    let your_player_id = search_another_player(&connection, &req_room_id, &req_player_id);

    for _i in 0..900 { //最大3分
        println!("{}", req_player_id);
        tokio::time::delay_for(Duration::from_millis(200)).await; //nミリ秒待つ
        if search_commands(&connection, Some(req_room_id), Some(your_player_id), Some(turn)).len() > 0 {
            break;
        }
    }

    //この後の処理一連をするフラグ取得
    let do_check_flg = check_done_turn(&connection, req_room_id);
    println!("{:?}がbreakしてフラグ{:?}", req_player_id, do_check_flg);
    if do_check_flg {
        println!("{:?}", "checkしない");
        for _i in 0..300 { //最大1分
            tokio::time::delay_for(Duration::from_millis(200)).await; //nミリ秒待つ
            if search_show_battles(&connection, Some(req_room_id)).len() > 0 { //処理が終わってたら
                break;
            }
        }
        println!("{:?}", "かえす");
        let show_battles = search_show_battles(&connection, Some(req_room_id));
        let pokemons = show_battles.iter() //なぜかmoveしない？
            .fold(Vec::new(), |mut acc, v| {
                if let 200 = v.kind {
                    acc.push(return_pokemon(&connection, v.pokemon_id));
                };
                acc
            });
        return HttpResponse::Ok().json((show_battles, pokemons))
    }

    let your_command = search_commands(&connection, Some(req_room_id), Some(your_player_id), Some(turn)).pop().unwrap();

    //それぞれのポケモン、持ち技の情報
    let my_pokemon_id = search_player_fields(&connection, Some(req_room_id), Some(req_player_id)).pop().unwrap().field_pokemon1_id;
    let your_pokemon_id = search_player_fields(&connection, Some(req_room_id), Some(your_player_id)).pop().unwrap().field_pokemon1_id;
    let my_pokemon = return_pokemon(&connection, my_pokemon_id);
    let your_pokemon = return_pokemon(&connection, your_pokemon_id);

    let my_battle_info2 = BattleInfo2::new(req_player_id, my_pokemon, my_command);
    let your_battle_info2 = BattleInfo2::new(your_player_id, your_pokemon, your_command);

    //素早さ比較
    let (mut first, mut second) = if check_faster(&my_battle_info2.pokemon, &your_battle_info2.pokemon) {(my_battle_info2, your_battle_info2)} else {(your_battle_info2, my_battle_info2)};

    //フロントに返すモーション
    let mut show_battles: Vec<ShowBattle> = vec![Default::default()];

    // let first_change_pokemon = 

    //先行交代処理
    if let 2 = first.command.command_type {
        let first_change_pokemon = return_pokemon(&connection, first.command.command_id);
        show_battles = change_pokemon(&connection, first.player_id, first_change_pokemon.made_pokemon.id, show_battles);
        first.pokemon = first_change_pokemon; //先行のポケモンを変更
    }
    //後攻交代処理
    if let 2 = second.command.command_type {
        let second_change_pokemon = return_pokemon(&connection, second.command.command_id);
        show_battles = change_pokemon(&connection, second.player_id, second_change_pokemon.made_pokemon.id, show_battles);
        second.pokemon = second_change_pokemon; //後攻のポケモンを変更
    }
    //先行攻撃処理
    if let 1 = first.command.command_type {
        let first_move = search_move_bases(&connection, Some(first.command.command_id)).pop().unwrap();
        show_battles = do_battle(&first, &second, &first_move, show_battles);
    }
    //後攻攻撃処理
    if let 1 = second.command.command_type {
        let second_move = search_move_bases(&connection, Some(second.command.command_id)).pop().unwrap();
        if let None = show_battles.iter().find(|&x| x.kind == 120) { //後攻が死んでない時のみ処理
            show_battles = do_battle(&second, &first, &second_move, show_battles);
        }
    }

    // if first.command.command_type == 2 { //先行が交代
    //     let first_change_pokemon = return_pokemon(&connection, first.command.command_id);
    //     show_battles = change_pokemon(&connection, first.player_id, first_change_pokemon.made_pokemon.id, show_battles);

    //     if second.command.command_type == 2 { //後攻も交代
    //         let second_change_pokemon = return_pokemon(&connection, second.command.command_id);
    //         show_battles = change_pokemon(&connection, second.player_id, second_change_pokemon.made_pokemon.id, show_battles);
    //     } else { //後攻は攻撃
    //         let second_move = search_move_bases(&connection, Some(second.command.command_id)).pop().unwrap();
    //         first.pokemon = first_change_pokemon; //先行のポケモンを変更
    //         show_battles = do_battle(&second, &first, &second_move, show_battles);
    //     }
    // } else { //先行が攻撃
    //     let first_move = search_move_bases(&connection, Some(first.command.command_id)).pop().unwrap();

    //     if (second.command.command_type == 2) { //後攻は交代
    //         let second_change_pokemon = return_pokemon(&connection, second.command.command_id);
    //         show_battles = change_pokemon(&connection, second.player_id, second_change_pokemon.made_pokemon.id, show_battles);

    //         second.pokemon = second_change_pokemon; //後攻のポケモンを変更
    //         show_battles = do_battle(&first, &second, &first_move, show_battles);
    //     } else { //後攻も攻撃
    //         let second_move = search_move_bases(&connection, Some(second.command.command_id)).pop().unwrap();
    //         show_battles = do_battle(&first, &second, &first_move, show_battles);
    //         if let None = show_battles.iter().find(|&x| x.kind == 120) { //後攻が死んでない時バトル処理
    //             show_battles = do_battle(&second, &first, &second_move, show_battles); //後攻バトル
    //         }        
    //     }
    // }

    //交換選択時
    //両方が交換選択
    // match (first.command.command_type, second.command.command_type) {
    //     (2, _) => {
    //         let first_change_pokemon = return_pokemon(&connection, first.command.command_id);
    //         show_battles = change_pokemon(&connection, first.player_id, first_change_pokemon.made_pokemon.id, show_battles);
    
    //     },
    //     (_, 2) => {
    //         let second_change_pokemon = return_pokemon(&connection, second.command.command_id);
    //         show_battles = change_pokemon(&connection, second.player_id, second_change_pokemon.made_pokemon.id, show_battles);
    //     },
    //     (_, _) => {

    //     }
    // };


    //両方がバトル選択

    //技検索
    // let first_move = search_move_bases(&connection, Some(first.command.command_id)).pop().unwrap();
    // let second_move = search_move_bases(&connection, Some(second.command.command_id)).pop().unwrap();
    // // let first_battle_info = BattleInfo::new(req_player_id, my_pokemon, my_move);
    // // let second_battle_info = BattleInfo::new(your_player_id, your_pokemon, your_move);

    // //素早さ比較
    // // let (first, second) = if check_faster(&my_battle_info.pokemon, &your_battle_info.pokemon) {(my_battle_info, your_battle_info)} else {(your_battle_info, my_battle_info)};

    // show_battles = do_battle(&first, &second, &first_move, show_battles); //先行バトル
    // if let None = show_battles.iter().find(|&x| x.kind == 120) { //後攻が死んでない時バトル処理
    //     show_battles = do_battle(&second, &first, &second_move, show_battles); //後攻バトル
    // }
    show_battles.iter_mut().map(|x| x.room_id = req_room_id).collect::<Vec<_>>();
    insert_show_battle(&connection, &show_battles);

    //     match (first.command.command_type, second.command.command_type) {
    //     (2, _) => {
    //         let first_change_pokemon = return_pokemon(&connection, first.command.command_id);
    //         show_battles = change_pokemon(&connection, first.player_id, first_change_pokemon.made_pokemon.id, show_battles);
    
    //     },
    //     (_, 2) => {
    //         let second_change_pokemon = return_pokemon(&connection, second.command.command_id);
    //         show_battles = change_pokemon(&connection, second.player_id, second_change_pokemon.made_pokemon.id, show_battles);
    //     },
    //     (_, _) => {

    //     }
    // };

    HttpResponse::Ok().json((show_battles, [first.pokemon, second.pokemon]))
}

//相手のポケモンが死んだときの交換待ち?
// #[get("/wait-your-death/{req_room_id}/{req_player_id}")] //room_idもつけた方がいい？
// async fn wait_your_death(web::Path((req_room_id, req_player_id)): web::Path<(i32, i32)>) -> impl Responder {
//     let connection = establish_connection();
//     //現在ターン取得
//     let turn = search_fields(&connection, Some(req_room_id)).pop().unwrap().turn;
//     //相手の情報取得
//     let your_player_id = search_another_player(&connection, &req_room_id, &req_player_id);
//     for _i in 0..900 { //最大3分
//         println!("{}", req_player_id);
//         tokio::time::delay_for(Duration::from_millis(200)).await; //nミリ秒待つ
//         if search_commands(&connection, Some(req_room_id), Some(your_player_id), Some(turn)).len() > 0 {
//             break;
//         }
//     }
// }



//ポケモン一匹の全情報
// #[get("/pokemon-and-relative/{value}")]
// async fn get_pokemon_and_relative(web::Path(value): web::Path<u32>) -> impl Responder {
//     println!("{}", value);
//     let connection = establish_connection();
//     let value: i32 = value as i32;

//     let search_made_pokemons = search_made_pokemons(&connection, value);
//     let search_base_pokemons = search_base_pokemons(&connection, search_made_pokemons[0].base_pokemon_id);
    
//     let search_battle_pokemon = p_in_battle_pokemons.filter(made_pokemon_id.eq(value))
//         .load::<BattlePokemon>(&connection)
//         .expect("Error loading p_in_battle_pokemons");

//     let mut moves: Vec<Vec<MoveBase>> = vec![Default::default()];
//     //技情報
//     moves.push(search_move_bases(&connection, search_made_pokemons[0].move1_id));
//     // moves.push(search_move_bases(&connection, search_made_pokemons[0].move2_id));
//     // moves.push(search_move_bases(&connection, search_made_pokemons[0].move3_id));
//     // moves.push(search_move_bases(&connection, search_made_pokemons[0].move4_id));
//     println!("{:?}", moves);

//     web::Json(
//         {ReturnPokemon 
//             {made_pokemon: search_made_pokemons,
//             base_pokemon: search_base_pokemons,
//             battle_pokemon: search_battle_pokemon,
//             moves: moves,
//         }}
//     )
// }

