table! {
    f_player_fields (player_id) {
        player_id -> Varchar,
        field_pokemon1_id -> Integer,
        field_pokemon2_id -> Integer,
    }
}

table! {
    m_move_bases (id) {
        id -> Integer,
        name -> Varchar,
        type_id -> Integer,
        max_pp -> Integer,
        accuracy -> Integer,
        power_v -> Integer,
        target -> Integer,
        order -> Integer,
        is_sound -> Integer,
        is_powder -> Integer,
        kind -> Integer,
    }
}

table! {
    p_base_pokemons (id) {
        id -> Integer,
        name -> Varchar,
        weight -> Float,
        type1_id -> Integer,
        type2_id -> Integer,
        ability1_id -> Integer,
        ability2_id -> Integer,
        hidden_ability_id -> Integer,
        gender_flg -> Integer,
        h_base -> Integer,
        a_base -> Integer,
        b_base -> Integer,
        c_base -> Integer,
        d_base -> Integer,
        s_base -> Integer,
    }
}

table! {
    p_in_battle_pokemons (made_pokemon_id) {
        made_pokemon_id -> Integer,
        hp_minus -> Integer,
        ailment -> Integer,
        a_updown -> Integer,
        b_updown -> Integer,
        c_updown -> Integer,
        d_updown -> Integer,
        s_updown -> Integer,
        acc_updown -> Integer,
        avoid_updown -> Integer,
        vital_updown -> Integer,
        move1_pp_minus -> Integer,
        move2_pp_minus -> Integer,
        move3_pp_minus -> Integer,
        move4_pp_minus -> Integer,
    }
}

table! {
    p_made_pokemons (id) {
        id -> Integer,
        player_id -> Integer,
        base_pokemon_id -> Integer,
        nickname -> Varchar,
        level -> Integer,
        gender_id -> Integer,
        ability_id -> Integer,
        nature_id -> Integer,
        item_id -> Integer,
        h_iv -> Integer,
        a_iv -> Integer,
        b_iv -> Integer,
        c_iv -> Integer,
        d_iv -> Integer,
        s_iv -> Integer,
        h_ev -> Integer,
        a_ev -> Integer,
        b_ev -> Integer,
        c_ev -> Integer,
        d_ev -> Integer,
        s_ev -> Integer,
        max_hp -> Integer,
        a_v -> Integer,
        b_v -> Integer,
        c_v -> Integer,
        d_v -> Integer,
        s_v -> Integer,
        move1_id -> Integer,
        move2_id -> Integer,
        move3_id -> Integer,
        move4_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
    }
}

table! {
    s_rooms (room_id) {
        id -> Integer,
        player1_id -> Integer,
        player2_id -> Integer,
        is_double_battle -> Integer,
        room_id -> Text,
        password -> Text,
    }
}

table! {
    s_players (id) {
        id -> Integer,
        player_id -> Varchar,
        password -> Varchar,
    }
}

table! {
    s_selected_pokemons (id) {
        id -> Integer,
        room_id -> Integer,
        player_id -> Integer,
        pokemon1_id -> Integer,
        pokemon2_id -> Integer,
        pokemon3_id -> Integer,
        pokemon4_id -> Integer,
        pokemon5_id -> Integer,
        pokemon6_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    f_player_fields,
    m_move_bases,
    p_base_pokemons,
    p_in_battle_pokemons,
    p_made_pokemons,
    posts,
    s_rooms,
    s_players,
);
