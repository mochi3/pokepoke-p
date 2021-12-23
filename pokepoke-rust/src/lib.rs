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

pub fn search_made_pokemons (connection: &MysqlConnection, _id: Option<i32>, _player_id: Option<i32>)  -> Vec<MadePokemon> {
    let mut query = schema::p_made_pokemons::table.order(schema::p_made_pokemons::id.desc()).into_boxed(); //並び替えてる？
    query = if let Some(x) = _id {query.filter(schema::p_made_pokemons::id.eq(x))} else {query};
    query = if let Some(x) = _player_id {query.filter(schema::p_made_pokemons::player_id.eq(x))} else {query};
    query.load::<MadePokemon>(connection).expect("Error loading posts")
}

pub fn search_made_pokemonss_by_selected_pokemons (connection: &MysqlConnection, pokemons: SelectedPokemon) -> Vec<MadePokemon> {
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

pub fn search_base_pokemons (connection: &MysqlConnection, _id: Option<i32>)  -> Vec<BasePokemon> {
    let mut query = schema::p_base_pokemons::table.into_boxed();
    query = if let Some(x) = _id {query.filter(schema::p_base_pokemons::id.eq(x))} else {query};
    query.load::<BasePokemon>(connection).expect("Error loading posts")
}

pub fn search_battle_pokemons (connection: &MysqlConnection, _made_pokemon_id: Option<i32>, _player_id: Option<i32>, _number: Option<i32>)  -> Vec<BattlePokemon> {
    let mut query = schema::p_in_battle_pokemons::table.into_boxed();
    query = if let Some(x) = _made_pokemon_id {query.filter(schema::p_in_battle_pokemons::made_pokemon_id.eq(x))} else {query};
    query = if let Some(x) = _player_id {query.filter(schema::p_in_battle_pokemons::player_id.eq(x))} else {query};
    query = if let Some(x) = _number {query.filter(schema::p_in_battle_pokemons::number.eq(x))} else {query};
    query.load::<BattlePokemon>(connection).expect("Error loading posts")
}

pub fn search_rooms (connection: &MysqlConnection, _id: Option<i32>) -> Vec<Room> {
    let mut query = schema::s_rooms::table.into_boxed();
    query = if let Some(x) = _id {query.filter(schema::s_rooms::id.eq(x))} else {query};
    query.load::<Room>(connection).expect("Error loading posts")
}

pub fn search_select_pokemons (connection: &MysqlConnection, _id: Option<i32>) -> Vec<SelectedPokemon> {
    let mut query = schema::s_selected_pokemons::table.into_boxed();
    query = if let Some(x) = _id {query.filter(schema::s_selected_pokemons::id.eq(x))} else {query};
    query.load::<SelectedPokemon>(connection).expect("Error loading posts")
}

pub fn search_move_bases (connection: &MysqlConnection, _id: Option<i32>) -> Vec<MoveBase> {
    let mut query = schema::m_move_bases::table.into_boxed();
    query = if let Some(x) = _id {query.filter(schema::m_move_bases::id.eq(x))} else {query};
    query.load::<MoveBase>(connection).expect("Error loading posts")
}

pub fn search_commands (connection: &MysqlConnection, _room_id: Option<i32>, _player_id: Option<i32>, _turn: Option<i32>) -> Vec<Command> {
    let mut query = schema::f_commands::table.into_boxed();
    query = if let Some(x) = _room_id {query.filter(schema::f_commands::room_id.eq(x))} else {query};
    query = if let Some(x) = _player_id {query.filter(schema::f_commands::player_id.eq(x))} else {query};
    query = if let Some(x) = _turn {query.filter(schema::f_commands::turn.eq(x))} else {query};
    query.load::<Command>(connection).expect("Error loading posts")
}

pub fn search_player_fields (connection: &MysqlConnection, _room_id: Option<i32>, _player_id: Option<i32>) -> Vec<PlayerField> {
    let mut query = schema::f_player_fields::table.into_boxed();
    query = if let Some(x) = _room_id {query.filter(schema::f_player_fields::room_id.eq(x))} else {query};
    query = if let Some(x) = _player_id {query.filter(schema::f_player_fields::player_id.eq(x))} else {query};
    query.load::<PlayerField>(connection).expect("Error loading posts")
}

pub fn search_fields (connection: &MysqlConnection, _room_id: Option<i32>) -> Vec<Field> {
    let mut query = schema::f_fields::table.into_boxed();
    query = if let Some(x) = _room_id {query.filter(schema::f_fields::room_id.eq(x))} else {query};
    query.load::<Field>(connection).expect("Error loading posts")
}

pub fn search_show_battles (connection: &MysqlConnection, _room_id: Option<i32>) -> Vec<ShowBattle> {
    let mut query = schema::f_show_battles::table.into_boxed();
    query = if let Some(x) = _room_id {query.filter(schema::f_show_battles::room_id.eq(x))} else {query};
    query.load::<ShowBattle>(connection).expect("Error loading posts")
}

//-----------------insert-------------------------------------

pub fn insert_made_pokemon (connection: &MysqlConnection, _value: &MadePokemon) {
    diesel::insert_into(schema::p_made_pokemons::table)
        .values(_value)
        .execute(connection)
        .expect("Error saving new post");
}

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
    let room = search_rooms(&connection, Some(*_room_id));
    if room[0].player1_id == *_player_id {room[0].player2_id} else {room[0].player1_id}
}

pub fn return_pokemon(connection: &MysqlConnection, _made_pokemon_id: i32) -> ReturnPokemon {
    let made_pokemon = search_made_pokemons(connection, Some(_made_pokemon_id), None).pop().unwrap();
    let base_pokemon = search_base_pokemons(connection, Some(made_pokemon.base_pokemon_id)).pop().unwrap();
    let battle_pokemon = search_battle_pokemons(&connection, Some(_made_pokemon_id), None, None).pop().unwrap();

    let mut moves: Vec<MoveBase> = Vec::new();
    if made_pokemon.move1_id != 0 {moves.push(search_move_bases(&connection, Some(made_pokemon.move1_id)).pop().unwrap())};
    if made_pokemon.move2_id != 0 {moves.push(search_move_bases(&connection, Some(made_pokemon.move2_id)).pop().unwrap())};
    if made_pokemon.move3_id != 0 {moves.push(search_move_bases(&connection, Some(made_pokemon.move3_id)).pop().unwrap())};
    if made_pokemon.move4_id != 0 {moves.push(search_move_bases(&connection, Some(made_pokemon.move4_id)).pop().unwrap())};

    ReturnPokemon {
        made_pokemon: made_pokemon,
        base_pokemon: base_pokemon,
        battle_pokemon: battle_pokemon,
        moves: moves,    
    }
}


pub fn change_pokemon(connection: &MysqlConnection, _player_id: i32, _made_pokemon_id: i32, mut show_battles:Vec<ShowBattle>) -> Vec<ShowBattle> {
    show_battles.push(ShowBattle {kind: 200, player_id:_player_id, pokemon_id:_made_pokemon_id, ..Default::default()});

    show_battles
}


pub fn check_done_turn(connection: &MysqlConnection, _room_id: i32) -> bool {
    let field = &search_fields(connection, Some(_room_id)).pop().unwrap();
    if field.turn > field.done_turn {
        update_field_done_turn(connection, _room_id, &field.turn);
        false
    } else {
        true
    }
}

//参照の方
// pub fn pokemon(connection: &MysqlConnection, _made_pokemon_id: i32) -> Pokemon {
//     let made_pokemon = search_made_pokemons(connection, _made_pokemon_id);
//     let base_pokemon = search_base_pokemons(connection, made_pokemon[0].base_pokemon_id);
//     let battle_pokemon = search_battle_pokemons(&connection, _made_pokemon_id, "made_pokemon_id");

//     Pokemon {
//         made_data: &made_pokemon[0],
//         base_data: &base_pokemon[0],
//         battle_data: &battle_pokemon[0],
//     }
// }

//------------------------------------------バトル用-----------------------------------------------------------------------------


//すばやさ比較
pub fn check_faster<'a>(pokemon1: &'a ReturnPokemon, pokemon2: &'a ReturnPokemon) -> bool {
    if pokemon1.made_pokemon.s_v == pokemon2.made_pokemon.s_v {
        thread_rng().gen_range(1..=2) == 1
    } else {
        pokemon1.made_pokemon.s_v > pokemon2.made_pokemon.s_v
    }
}

//バトル
pub fn do_battle<'a>(atk_info: &BattleInfo2, def_info: &BattleInfo2, moving: &MoveBase, mut show_battles: Vec<ShowBattle>)
     -> Vec<ShowBattle> {
    let mut death_flg = false;
    //攻撃使用
    show_battles.push(ShowBattle {kind: 100, pokemon_id: atk_info.pokemon.made_pokemon.id, player_id:atk_info.player_id, ..ShowBattle::new(&moving.name)});
    //命中判定
    let hit = hit_check(&atk_info.pokemon, &def_info.pokemon, &moving);
    if !hit { //外したとき処理終了
        show_battles.push(ShowBattle {kind:110, pokemon_id:atk_info.pokemon.made_pokemon.id, player_id:atk_info.player_id, ..Default::default()});
        return show_battles;
    }
    //ダメージ計算
    let damage = from_pokemon_damage_calculate(&atk_info.pokemon, &def_info.pokemon, &moving);
    println!("初期ダメージ：{:?}", damage);
    //急所
    let (critical, damage, show_battles) = check_critical_hit(&atk_info.pokemon.battle_pokemon.vital_updown, &damage, show_battles, &def_info);
    //タイプ一致
    let (type_match, damage) = from_pokemon_check_type_match(&atk_info.pokemon, &moving, damage);
    //タイプ相性
    let (type_compatibility, damage, mut show_battles) = from_pokemon_check_type_compatibility(&def_info.pokemon, &moving, damage, show_battles, &def_info);
    println!("タイプ計算後ダメージ：{:?}", damage);
    //ダメージ演出追加
    show_battles.push(ShowBattle {kind: 101, pokemon_id:def_info.pokemon.made_pokemon.id, value: damage, player_id:def_info.player_id, ..Default::default()});
    //死亡判定
    if def_info.pokemon.made_pokemon.max_hp < def_info.pokemon.battle_pokemon.hp_minus + damage { //死亡したとき
        show_battles.push(ShowBattle {kind: 120, pokemon_id:def_info.pokemon.made_pokemon.id, player_id:def_info.player_id, ..Default::default()});
        death_flg = true;
    }

    show_battles
}

fn hit_check(atk_pokemon: &ReturnPokemon, def_pokemon: &ReturnPokemon, moving: &MoveBase) -> bool {
    let rand = thread_rng().gen_range(0..=99);
    let mut hit_rank = 1.0;
    if moving.accuracy != 30 {
        hit_rank = check_hit_rank(atk_pokemon.battle_pokemon.acc_updown - atk_pokemon.battle_pokemon.avoid_updown);
    };
    println!("{:?}が{:?}未満かで命中計算", rand, (moving.accuracy as f32 * hit_rank) as i32);
    rand < ((moving.accuracy as f32 * hit_rank) as i32)
}

fn from_pokemon_damage_calculate(atk_pokemon: &ReturnPokemon, def_pokemon: &ReturnPokemon, moving: &MoveBase) -> i32 {
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
fn check_critical_hit(rank: &i32, damage: &i32, mut show_battles: Vec<ShowBattle>, def_info: &BattleInfo2) -> (bool, i32, Vec<ShowBattle>) {
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
fn from_pokemon_check_type_match(atk_pokemon: &ReturnPokemon, moving: &MoveBase, damage: i32) -> (bool, i32) {
    if atk_pokemon.base_pokemon.type1_id | atk_pokemon.base_pokemon.type2_id == moving.type_id {
        (true, (damage as f32 * 1.5) as i32)
    } else {
        (false, damage)
    }
}

//タイプ相性前処理
fn from_pokemon_check_type_compatibility(def_pokemon: &ReturnPokemon, moving: &MoveBase, damage: i32, mut show_battles: Vec<ShowBattle>, def_info: &BattleInfo2)
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

//命中ランク
fn check_hit_rank(hit_rank: i32) -> f32 {
    match hit_rank {
        -5 => 3.0/8.0,
        -4 => 3.0/7.0,
        -3 => 1.0/2.0,
        -2 => 3.0/5.0,
        -1 => 3.0/4.0,
        0 => 1.0,
        1 => 4.0/3.0,
        2 => 5.0/3.0,
        3 => 2.0,
        4 => 7.0/3.0,
        5 => 8.0/3.0,
        _ => if hit_rank > 6 {3.0} else {1.0/3.0}
    }
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
