#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use rand::prelude::*; //乱数生成用

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
use schema::f_commands::dsl::*;
use schema::f_player_fields::dsl::*;
use schema::f_fields::dsl::*;
use schema::f_show_battles::dsl::*;

//-----------select---------------------------------

pub fn search_made_pokemon (connection: &MysqlConnection, _value: i32)  -> Vec<MadePokemon> {
    p_made_pokemons.filter(schema::p_made_pokemons::id.eq(_value))
        .load::<MadePokemon>(connection)
        .expect("Error loading posts")
}

pub fn search_base_pokemon (connection: &MysqlConnection, _value: i32)  -> Vec<BasePokemon> {
    p_base_pokemons.filter(schema::p_base_pokemons::id.eq(_value))
        .load::<BasePokemon>(connection)
        .expect("Error loading posts")
}

pub fn search_battle_pokemons (connection: &MysqlConnection, _value: i32, column: &str)  -> Vec<BattlePokemon> {
    if column == "made_pokemon_id" {
        p_in_battle_pokemons.filter(schema::p_in_battle_pokemons::made_pokemon_id.eq(_value))
            .load::<BattlePokemon>(connection)
            .expect("Error loading posts")
    } else if column == "player_id" {
        p_in_battle_pokemons.filter(schema::p_in_battle_pokemons::player_id.eq(_value))
            .load::<BattlePokemon>(connection)
            .expect("Error loading posts")
    } else {
        p_in_battle_pokemons.filter(schema::p_in_battle_pokemons::player_id.eq(_value))
            .load::<BattlePokemon>(connection)
            .expect("Error loading posts")
    }
}

pub fn search_battle_pokemons_first (connection: &MysqlConnection, _player_id: i32)  -> Vec<BattlePokemon> {
    p_in_battle_pokemons.filter(schema::p_in_battle_pokemons::player_id.eq(_player_id))
        .filter(schema::p_in_battle_pokemons::number.eq(1))
        .load::<BattlePokemon>(connection)
        .expect("Error loading posts")
}

pub fn search_room (connection: &MysqlConnection, _value: i32) -> Vec<Room> {
    s_rooms.filter(schema::s_rooms::id.eq(_value))
        .load::<Room>(connection)
        .expect("Error loading posts")
}

pub fn search_select_pokemon (connection: &MysqlConnection, _value: i32) -> Vec<SelectedPokemon> {
    s_selected_pokemons.filter(schema::s_selected_pokemons::id.eq(_value))
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

pub fn search_move_base (connection: &MysqlConnection, _value: i32) -> Vec<MoveBase> {
    m_move_bases.filter(schema::m_move_bases::id.eq(_value))
        .load::<MoveBase>(connection)
        .expect("Error loading posts")
}

pub fn search_command (connection: &MysqlConnection, _room_id: i32, _player_id: i32, _turn: i32) -> Vec<Command> {
    f_commands
        .filter(schema::f_commands::room_id.eq(_room_id))
        .filter(schema::f_commands::player_id.eq(_player_id))
        .filter(schema::f_commands::turn.eq(_turn))
        .load::<Command>(connection)
        .expect("Error loading posts")
}

pub fn search_field_pokemon (connection: &MysqlConnection, _room_id: i32, _player_id: i32) -> i32 {
    f_player_fields
        .filter(schema::f_player_fields::room_id.eq(_room_id))
        .filter(schema::f_player_fields::player_id.eq(_player_id))
        .load::<PlayerField>(connection)
        .expect("Error loading posts")[0].field_pokemon1_id
}

pub fn search_field (connection: &MysqlConnection, _room_id: i32) -> Vec<Field> {
    f_fields
        .filter(schema::f_fields::room_id.eq(_room_id))
        .load::<Field>(connection)
        .expect("Error loading posts")
}

pub fn search_show_battles (connection: &MysqlConnection, _room_id: i32) -> Vec<ShowBattle> {
    f_show_battles
        .filter(schema::f_show_battles::room_id.eq(_room_id))
        .load::<ShowBattle>(connection)
        .expect("Error loading posts")
}

//-----------------insert-------------------------------------

pub fn insert_room (connection: &MysqlConnection, _value: &Room) {
    diesel::insert_into(schema::s_rooms::table)
        .values(_value)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn insert_selected_pokemon (connection: &MysqlConnection, _value: &SelectedPokemon) {
    diesel::insert_into(schema::s_selected_pokemons::table)
        .values(_value)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn insert_battle_pokemon (connection: &MysqlConnection, _value: &BattlePokemon) {
    diesel::insert_into(schema::p_in_battle_pokemons::table)
        .values(_value)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn insert_command (connection: &MysqlConnection, _value: &Command) {
    diesel::insert_into(schema::f_commands::table)
        .values(_value)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn insert_show_battle (connection: &MysqlConnection, _value: &Vec<ShowBattle>) {
    diesel::insert_into(schema::f_show_battles::table)
        .values(_value)
        .execute(connection)
        .expect("Error saving new post");
}


//---------------------update------------------------------------------------
pub fn update_field_done_turn (connection: &MysqlConnection, _room_id: i32, _turn: &i32) {
    diesel::update(f_fields.filter(schema::f_fields::room_id.eq(_room_id)))
        .set(schema::f_fields::done_turn.eq(_turn))
        .execute(connection)
        .expect("Error saving new post");
}


//---------------その他DB------------------------------------------

pub fn search_another_player (connection: &MysqlConnection, _room_id: &i32, _player_id: &i32) -> i32 {
    let room = search_room(&connection, *_room_id);
    if room[0].player1_id == *_player_id {room[0].player2_id} else {room[0].player1_id}
}

pub fn return_pokemon(connection: &MysqlConnection, _made_pokemon_id: i32) -> ReturnPokemon1 {
    let made_pokemon = search_made_pokemon(connection, _made_pokemon_id).pop().unwrap();
    let base_pokemon = search_base_pokemon(connection, made_pokemon.base_pokemon_id).pop().unwrap();
    let battle_pokemon = search_battle_pokemons(&connection, _made_pokemon_id, "made_pokemon_id").pop().unwrap();

    let mut moves: Vec<MoveBase> = Vec::new();
    if made_pokemon.move1_id != 0 {moves.push(search_move_base(&connection, made_pokemon.move1_id).pop().unwrap())};
    if made_pokemon.move2_id != 0 {moves.push(search_move_base(&connection, made_pokemon.move2_id).pop().unwrap())};
    if made_pokemon.move3_id != 0 {moves.push(search_move_base(&connection, made_pokemon.move3_id).pop().unwrap())};
    if made_pokemon.move4_id != 0 {moves.push(search_move_base(&connection, made_pokemon.move4_id).pop().unwrap())};

    ReturnPokemon1 {
        made_pokemon: made_pokemon,
        base_pokemon: base_pokemon,
        battle_pokemon: battle_pokemon,
        moves: moves,    
    }
}

pub fn check_done_turn(connection: &MysqlConnection, _room_id: i32) -> bool {
    let field = &search_field(connection, _room_id)[0];
    if field.turn > field.done_turn {
        update_field_done_turn(connection, _room_id, &field.turn);
        false
    } else {
        true
    }
}

//参照の方
// pub fn pokemon(connection: &MysqlConnection, _made_pokemon_id: i32) -> Pokemon {
//     let made_pokemon = search_made_pokemon(connection, _made_pokemon_id);
//     let base_pokemon = search_base_pokemon(connection, made_pokemon[0].base_pokemon_id);
//     let battle_pokemon = search_battle_pokemons(&connection, _made_pokemon_id, "made_pokemon_id");

//     Pokemon {
//         made_data: &made_pokemon[0],
//         base_data: &base_pokemon[0],
//         battle_data: &battle_pokemon[0],
//     }
// }

//------------------------------------------バトル用-----------------------------------------------------------------------------


//すばやさ比較
pub fn check_faster<'a>(pokemon1: &'a ReturnPokemon1, pokemon2: &'a ReturnPokemon1)
     -> bool {
    pokemon1.made_pokemon.s_v > pokemon2.made_pokemon.s_v //素早さ同じときの処理かく
}

//バトル
pub fn do_battle<'a>(atk_info: &BattleInfo, def_info: &BattleInfo, mut show_battles: Vec<ShowBattle>)
     -> Vec<ShowBattle> {
    //攻撃使用
    show_battles.push(ShowBattle {kind: 100, pokemon_id: atk_info.pokemon.made_pokemon.id, player_id:atk_info.player_id, ..ShowBattle::new(&atk_info.moving.name)});
    // ダメージ計算
    let damage = from_pokemon_damage_calculate(&atk_info.pokemon, &def_info.pokemon, &atk_info.moving);
    println!("{:?}", damage);
    //急所
    let (critical, damage, show_battles) = check_critical_hit(&atk_info.pokemon.battle_pokemon.vital_updown, &damage, show_battles, &def_info);
    //タイプ一致
    let (type_match, damage) = from_pokemon_check_type_match(&atk_info.pokemon, &atk_info.moving, damage);
    // タイプ相性
    let (type_compatibility, damage, mut show_battles) = from_pokemon_check_type_compatibility(&def_info.pokemon, &atk_info.moving, damage, show_battles, &def_info);
    println!("{:?}", damage);
    //ダメージ入力
    show_battles.push(ShowBattle {kind: 101, pokemon_id:  def_info.pokemon.made_pokemon.id, value: damage, player_id:def_info.player_id, ..Default::default()});

    show_battles
}

fn from_pokemon_damage_calculate(atk_pokemon: &ReturnPokemon1, def_pokemon: &ReturnPokemon1, moving: &MoveBase) -> i32 {
    damage_calculate(&atk_pokemon.made_pokemon.level, 
        if moving.kind==1 {&atk_pokemon.made_pokemon.a_v} else {&atk_pokemon.made_pokemon.c_v},
        if moving.kind==1 {&def_pokemon.made_pokemon.b_v} else {&def_pokemon.made_pokemon.d_v},
        &moving.power_v)
}

fn damage_calculate(atk_level: &i32, atk_value: &i32, def_value: &i32, move_power: &i32) -> i32 {
    (((atk_level*2/5+2)
    *move_power *atk_value /def_value
    /50+2) as f32
    *(thread_rng().gen_range(85..=100)) as f32/100.0) as i32
}

//急所
fn check_critical_hit(rank: &i32, damage: &i32, mut show_battles: Vec<ShowBattle>, def_info: &BattleInfo) -> (bool, i32, Vec<ShowBattle>) {
    let rand = thread_rng().gen_range(1..=24);
    // let rand = 1; //デバッグ用:急所にあてる
    println!("急所：{:?}", rand);
    if rand == 1 || (rand <= 3 && *rank >= 1 ) || (rand <= 12 && *rank >= 2) || (*rank >= 3) { //急所に当たったとき
        show_battles.push(ShowBattle {kind: 102, player_id: def_info.player_id, ..Default::default()});
        (true, (*damage as f32 * 1.5) as i32, show_battles)
    } else {
        (false, *damage, show_battles)
    }
}

//タイプ一致
fn from_pokemon_check_type_match(atk_pokemon: &ReturnPokemon1, moving: &MoveBase, damage: i32) -> (bool, i32) {
    if atk_pokemon.base_pokemon.type1_id | atk_pokemon.base_pokemon.type2_id == moving.type_id {
        (true, (damage as f32 * 1.5) as i32)
    } else {
        (false, damage)
    }
}

//タイプ相性前処理
fn from_pokemon_check_type_compatibility(def_pokemon: &ReturnPokemon1, moving: &MoveBase, damage: i32, mut show_battles: Vec<ShowBattle>, def_info: &BattleInfo)
     -> (f32, i32, Vec<ShowBattle>) {
    let type_compatibility = check_type_compatibility(&moving.type_id, &def_pokemon.base_pokemon.type1_id)*check_type_compatibility(&moving.type_id, &def_pokemon.base_pokemon.type2_id);
    let _kind = match type_compatibility {
        0.5 | 0.25 => 104, //少数の書き方あってるかわからない
        2.0 | 4.0 => 105,
        0.0 => 106,
        _ => 0,
    };
    if _kind != 0 {
        show_battles.push(ShowBattle {kind: _kind, player_id: def_info.player_id, ..Default::default()});
    }
    (type_compatibility, (damage as f32 * type_compatibility) as i32, show_battles)
}

//タイプ相性
fn check_type_compatibility(atk_type: &i32, def_type: &i32) -> f32 {
    match (&atk_type, &def_type) {
        (1, 13) => 0.5, //ノーマル
        (1, 14) => 0.0,
        (1, 17) => 0.5,
        (2, 2) => 0.5, //ほのお
        (2, 3) => 0.5,
        (2, 5) => 2.0,
        (2, 6) => 2.0,
        (2, 12) => 2.0,
        (2, 13) => 0.5,
        (2, 15) => 0.5,
        (2, 17) => 2.0,
        (3, 2) => 2.0, //みず
        (3, 3) => 0.5,
        (3, 5) => 0.5,
        (3, 9) => 2.0,
        (3, 13) => 2.0,
        (3, 15) => 0.5,
        (4, 3) => 2.0, //でんき
        (4, 4) => 0.5,
        (4, 5) => 0.5,
        (4, 9) => 0.0,
        (4, 10) => 2.0,
        (4, 15) => 0.5,
        (5, 2) => 0.5, //くさ
        (5, 3) => 2.0,
        (5, 5) => 0.5,
        (5, 8) => 0.5,
        (5, 9) => 2.0,
        (5, 10) => 0.5,
        (5, 12) => 0.5,
        (5, 13) => 2.0,
        (5, 15) => 0.5,
        (5, 17) => 0.5,
        (6, 2) => 0.5, //こおり
        (6, 3) => 0.5,
        (6, 5) => 2.0,
        (6, 9) => 2.0,
        (6, 10) => 2.0,
        (6, 15) => 2.0,
        (6, 17) => 0.5,
        (7, 1) => 2.0, //かくとう
        (7, 6) => 2.0,
        (7, 8) => 0.5,
        (7, 10) => 0.5,
        (7, 11) => 0.5,
        (7, 12) => 0.5,
        (7, 13) => 2.0,
        (7, 14) => 0.0,
        (7, 16) => 2.0,
        (7, 17) => 2.0,
        (7, 18) => 0.5,
        (8, 5) => 2.0, //どく
        (8, 8) => 0.5,
        (8, 9) => 0.5,
        (8, 13) => 0.5,
        (8, 14) => 0.5,
        (8, 17) => 0.0,
        (8, 18) => 2.0,
        (9, 2) => 2.0, //じめん
        (9, 4) => 2.0,
        (9, 8) => 2.0,
        (9, 10) => 0.0,
        (9, 12) => 0.5,
        (9, 13) => 2.0,
        (9, 17) => 2.0,
        (10, 4) => 0.5, //ひこう
        (10, 5) => 2.0,
        (10, 7) => 2.0,
        (10, 12) => 2.0,
        (10, 13) => 0.5,
        (10, 17) => 0.5,
        (11, 7) => 2.0, //エスパー
        (11, 8) => 2.0,
        (11, 11) => 2.0,
        (11, 16) => 2.0,
        (11, 17) => 0.5,
        (12, 2) => 0.5, //むし
        (12, 5) => 2.0,
        (12, 7) => 0.5,
        (12, 8) => 0.5,
        (12, 10) => 0.5,
        (12, 11) => 2.0,
        (12, 14) => 0.5,
        (12, 16) => 2.0,
        (12, 17) => 0.5,
        (12, 18) => 0.5,
        (13, 2) => 2.0, //いわ
        (13, 6) => 2.0,
        (13, 7) => 0.5,
        (13, 9) => 0.5,
        (13, 10) => 2.0,
        (13, 12) => 2.0,
        (13, 17) => 0.5,
        (14, 1) => 0.0, //ゴースト
        (14, 11) => 2.0,
        (14, 14) => 2.0,
        (14, 16) => 0.5,
        (15, 15) => 2.0, //ドラゴン
        (15, 17) => 0.5,
        (15, 18) => 0.0,
        (16, 7) => 0.5, //あく
        (16, 11) => 2.0,
        (16, 14) => 2.0,
        (16, 16) => 0.5,
        (16, 18) => 0.5,
        (17, 2) => 0.5, //はがね
        (17, 3) => 0.5,
        (17, 4) => 0.5,
        (17, 6) => 2.0,
        (17, 13) => 2.0,
        (17, 17) => 0.5,
        (17, 18) => 2.0,
        (18, 2) => 0.5, //フェアリー
        (18, 7) => 2.0,
        (18, 8) => 0.5,
        (18, 15) => 2.0,
        (18, 16) => 2.0,
        (18, 17) => 0.5,
        (_, _) => 1.0
    }
}
