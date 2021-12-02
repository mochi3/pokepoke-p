extern crate pokepoke_rust;
extern crate diesel;

use self::pokepoke_rust::*;
use self::models::*;
use self::diesel::prelude::*;
use std::io::{stdin, Read};

use rand::prelude::*; //乱数生成用



fn main() {
    // println!("フロントで持っているポケモンid_一人目");
    // let mut pokemon_1 = String::new();
    // stdin().read_line(&mut pokemon_1).unwrap();
    // let pokemon_1 = &pokemon_1[..(pokemon_1.len() - 1)];

    // println!("技番号選択");
    // let mut selected_move_1 = String::new();
    // stdin().read_line(&mut selected_move_1).unwrap();
    // let selected_move_1 = &selected_move_1[..(selected_move_1.len() - 1)];

    // println!("フロントで持っているポケモンid_二人目");
    // let mut pokemon_2 = String::new();
    // stdin().read_line(&mut pokemon_2).unwrap();
    // let pokemon_2 = &pokemon_2[..(pokemon_2.len() - 1)];

    // println!("技番号選択");
    // let mut selected_move_2 = String::new();
    // stdin().read_line(&mut selected_move_2).unwrap();
    // let selected_move_2 = &selected_move_2[..(selected_move_2.len() - 1)];

    // println!("ポケモン1:{}, 技:{}\nポケモン2:{}, 技:{}", pokemon_1,selected_move_1,pokemon_2,selected_move_2);

    //db接続
    use pokepoke_rust::schema::p_made_pokemons::dsl::*;
    use pokepoke_rust::schema::p_base_pokemons::dsl::*;
    use pokepoke_rust::schema::p_in_battle_pokemons::dsl::*;
    use pokepoke_rust::schema::m_move_bases::dsl::*;

    let connection = establish_connection();
    // let pokemon_1 = pokemon_1.parse().unwrap();
    // let pokemon_2 = pokemon_2.parse().unwrap();
    // let selected_move_1 = selected_move_1.parse().unwrap();
    // let selected_move_2 = selected_move_2.parse().unwrap();
    let pokemon_1 = 1;
    let pokemon_2 = 2;
    let selected_move_1 = 1;
    let selected_move_2 = 5;
    
    //作ったポケモンを検索
    let search_made_pokemon = |x:i32| p_made_pokemons.filter(pokepoke_rust::schema::p_made_pokemons::id.eq(x))
        .limit(5)
        .load::<MadePokemon>(&connection)
        .expect("Error loading posts");
    let made_pokemon_data_1 = &search_made_pokemon(pokemon_1)[0];
    let made_pokemon_data_2 = &search_made_pokemon(pokemon_2)[0];

    //ポケモンの基礎データ検索
    let search_base_pokemon = |x:i32| p_base_pokemons.filter(pokepoke_rust::schema::p_base_pokemons::id.eq(x))
        .limit(5)
        .load::<BasePokemon>(&connection)
        .expect("Error loading posts");
    let base_pokemon_data_1 = &search_base_pokemon(made_pokemon_data_1.base_pokemon_id)[0];
    let base_pokemon_data_2 = &search_base_pokemon(made_pokemon_data_2.base_pokemon_id)[0];
    println!("ベースポケモン1：{:?}\nベースポケモン2：{:?}",base_pokemon_data_1,base_pokemon_data_2);

    // バトル中のポケモンを検索
    let search_battle_pokemon = |x:i32| p_in_battle_pokemons.filter(made_pokemon_id.eq(x))
        .limit(5)
        .load::<InBattlePokemon>(&connection)
        .expect("Error loading posts")[0];
    let battle_pokemon_data_1 = search_battle_pokemon(pokemon_1);
    let battle_pokemon_data_2 = search_battle_pokemon(pokemon_2);

    //ポケモンの情報まとめる
    let pokemon1: Pokemon = Pokemon::new(made_pokemon_data_1, base_pokemon_data_1, &battle_pokemon_data_1);
    let pokemon2: Pokemon = Pokemon::new(made_pokemon_data_2, base_pokemon_data_2, &battle_pokemon_data_2);
    println!("ポケモン1：{:?}\nポケモン2：{:?}",pokemon1,pokemon2);

    //技検索
    let search_move_base = |x:i32| m_move_bases.filter(pokepoke_rust::schema::m_move_bases::id.eq(x))
        .limit(5)
        .load::<MoveBase>(&connection)
        .expect("Error loading posts");
    let move_1 = &search_move_base(selected_move_1)[0]; //文字列はコピーできない（[0]できない）ので、&で文字列スライスを作る
    let move_2 = &search_move_base(selected_move_2)[0];

    println!("技１：{:?}\n技２：{:?}",move_1,move_2);

    // ダメージ計算
    let mut damage1 = from_pokemon_damage_calculate(&pokemon1, &pokemon2, &move_1);
    let mut damage2 = from_pokemon_damage_calculate(&pokemon2, &pokemon1, &move_2);
    println!("ダメージ１：{:?}、ダメージ２：{:?}", damage1, damage2);

    //急所
    let critical_1 = check_critical_hit(&pokemon1.battle_data.vital_updown);
    let critical_2 = check_critical_hit(&pokemon2.battle_data.vital_updown);
    println!("急所１：{:?}、急所２：{:?}", critical_1, critical_2);
    damage1 = (damage1 as f32 * if critical_1 {1.5} else {1.0}) as i32;
    damage2 = (damage2 as f32 * if critical_2 {1.5} else {1.0}) as i32;

    //タイプ一致
    let type_match_1 = from_pokemon_check_type_match(&pokemon1, &move_1);
    let type_match_2 = from_pokemon_check_type_match(&pokemon2, &move_2);
    println!("一致１：{:?}、一致２：{:?}", type_match_1, type_match_2);
    damage1 = (damage1 as f32 * if type_match_1 {1.5} else {1.0}) as i32;
    damage2 = (damage2 as f32 * if type_match_2 {1.5} else {1.0}) as i32;

    // タイプ相性（enumでmatchの方がいいかも）
    let type_compatibility_1 = from_pokemon_check_type_compatibility(&pokemon2, &move_1);
    let type_compatibility_2 = from_pokemon_check_type_compatibility(&pokemon1, &move_2);
    println!("相性１：{:?}、相性２：{:?}", type_compatibility_1, type_compatibility_2);
    damage1 = (damage1 as f32 * type_compatibility_1) as i32;
    damage2 = (damage2 as f32 * type_compatibility_2) as i32;

    //体力から引く
    // let remain_hp_pokemon2 = pokemon2.battle_data.hp_minus - damage1;
    // if remain_hp_pokemon2 <= 0 {
    // };

    println!("最終ダメージ１：{:?}、最終ダメージ２：{:?}", damage1, damage2);

}


fn from_pokemon_damage_calculate(atk_pokemon: &Pokemon, def_pokemon: &Pokemon, moving: &MoveBase) -> i32 {
    damage_calculate(&atk_pokemon.made_data.level, 
        if moving.kind==1 {&atk_pokemon.made_data.a_v} else {&atk_pokemon.made_data.c_v},
        if moving.kind==1 {&def_pokemon.made_data.b_v} else {&def_pokemon.made_data.d_v},
        &moving.power_v)
}

fn damage_calculate(atk_level: &i32, atk_value: &i32, def_value: &i32, move_power: &i32) -> i32 {
    (((atk_level*2/5+2)
    *move_power *atk_value /def_value
    /50+2) as f32
    *(thread_rng().gen_range(85..=100)) as f32/100.0) as i32
}



//ポケモンの情報まとめる用
#[derive(Debug)]
struct Pokemon<'a> {
    made_data: &'a MadePokemon,
    base_data: &'a BasePokemon,
    battle_data: &'a InBattlePokemon,
}
impl Pokemon<'_> {
    fn new<'a>(made_data: &'a MadePokemon, base_data: &'a BasePokemon, battle_data: &'a InBattlePokemon) -> Pokemon<'a> {
        Pokemon {
            made_data,
            base_data,
            battle_data,
        }
    }
}

//急所
fn check_critical_hit(rank: &i32) -> bool {
    let rand = thread_rng().gen_range(1..=24);
    println!("急所：{:?}", rand);
    rand == 1 || (rand <= 3 && *rank >= 1 ) || (rand <= 12 && *rank >= 2) || (*rank >= 3)
}

//タイプ一致
fn from_pokemon_check_type_match(atk_pokemon: &Pokemon, moving: &MoveBase) -> bool {
    atk_pokemon.base_data.type1_id | atk_pokemon.base_data.type2_id == moving.type_id
}

//タイプ相性（ポケモンから）
fn from_pokemon_check_type_compatibility(def_pokemon: &Pokemon, moving: &MoveBase) -> f32 {
    check_type_compatibility(&moving.type_id, &def_pokemon.base_data.type1_id)
    *check_type_compatibility(&moving.type_id, &def_pokemon.base_data.type2_id)
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
