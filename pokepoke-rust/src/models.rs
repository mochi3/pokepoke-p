// use super::schema::posts;
use super::schema::*;
use serde::{Deserialize, Serialize}; //シリアライズ(get:struct→json)、デシリアライズ(post:json→struct)

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
}

// #[derive(Deserialize, Insertable)]
// #[table_name="posts"]
// pub struct NewPost<'a> {
//     pub id: &'a i32,
//     pub title: &'a str,
// }
// #[derive(Serialize, Deserialize, Insertable)] //web接続用に書き直し
// #[table_name="posts"]
// pub struct NewPost {
//     pub id: i32,
//     pub title: String,
// }

#[derive(Queryable, Debug, Serialize, Default)]
pub struct MadePokemon {
    pub id: i32,
    pub player_id: i32,
    pub base_pokemon_id: i32,
    pub nickname: String,
    pub level: i32,
    pub gender_id: i32,
    pub ability_id: i32,
    pub nature_id: i32,
    pub item_id: i32,
    pub h_iv: i32,
    pub a_iv: i32,
    pub b_iv: i32,
    pub c_iv: i32,
    pub d_iv: i32,
    pub s_iv: i32,
    pub h_ev: i32,
    pub a_ev: i32,
    pub b_ev: i32,
    pub c_ev: i32,
    pub d_ev: i32,
    pub s_ev: i32,
    pub max_hp: i32,
    pub a_v: i32,
    pub b_v: i32,
    pub c_v: i32,
    pub d_v: i32,
    pub s_v: i32,
    pub move1_id: i32,
    pub move2_id: i32,
    pub move3_id: i32,
    pub move4_id: i32,
}

#[derive(Queryable, Debug, Clone, Serialize)]
pub struct BasePokemon {
    pub id: i32,
    pub name: String,
    pub weight: f32,
    pub type1_id: i32,
    pub type2_id: i32,
    pub ability1_id: i32,
    pub ability2_id: i32,
    pub hidden_ability_id: i32,
    pub gender_flg: i32,
    pub h_base: i32,
    pub a_base: i32,
    pub b_base: i32,
    pub c_base: i32,
    pub d_base: i32,
    pub s_base: i32,
}

#[derive(Queryable, Debug, Copy, Clone, Serialize)]
pub struct InBattlePokemon {
    pub made_pokemon_id: i32,
    pub hp_minus: i32,
    pub ailment: i32,
    pub a_updown: i32,
    pub b_updown: i32,
    pub c_updown: i32,
    pub d_updown: i32,
    pub s_updown: i32,
    pub acc_updown: i32,
    pub avoid_updown: i32,
    pub vital_updown: i32,
    pub move1_pp_minus: i32,
    pub move2_pp_minus: i32,
    pub move3_pp_minus: i32,
    pub move4_pp_minus: i32,
}

#[derive(Queryable, Debug, Default, Serialize)]
pub struct MoveBase {
    pub id: i32,
    pub name: String,
    pub type_id: i32,
    pub max_pp: i32,
    pub accuracy: i32,
    pub power_v: i32,
    pub target: i32,
    pub order: i32,
    pub is_sound: i32,
    pub is_powder: i32,
    pub kind: i32,
}

#[derive(Queryable, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct Player {
    pub id: i32,
    pub player_id: String,
    pub password: String,
}

// #[derive(Deserialize, Insertable, Debug)]
// // #[derive(Insertable)]
// #[table_name="s_rooms"]
// pub struct Room<'a> {
//     pub id: &'a i32,
//     pub player1_id: &'a i32,
//     pub player2_id: &'a i32,
//     pub is_double_battle: &'a i32,
//     pub room_id: &'a str,
//     pub password: &'a str,
// }
#[derive(Queryable, Deserialize, Serialize, Insertable, Debug, Eq, PartialEq)]
#[table_name="s_rooms"]
pub struct Room {
    pub id: i32,
    pub player1_id: i32,
    pub player2_id: i32,
    pub is_double_battle: i32,
    pub room_id: String,
    pub password: String,
}

#[derive(Queryable, Deserialize, Serialize, Insertable, Debug)]
#[table_name="s_selected_pokemons"]
pub struct SelectedPokemon {
    pub id: i32,
    pub room_id: i32,
    pub player_id: i32,
    pub pokemon1_id: i32,
    pub pokemon2_id: i32,
    pub pokemon3_id: i32,
    pub pokemon4_id: i32,
    pub pokemon5_id: i32,
    pub pokemon6_id: i32,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub id: &'a i32,
    pub title: &'a str,
    // pub body: &'a str,
}