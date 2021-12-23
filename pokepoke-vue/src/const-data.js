// const { h } = require("@vue/runtime-core");

module.exports = {
    TYPES: [
        {id: 1, name: "ノーマル", color: "#cbcbc3"},
        {id: 2, name: "ほのお", color: "#f5a179"},
        {id: 3, name: "みず", color: "#75c9f1"},
        {id: 4, name: "でんき", color: "#e7e364"},
        {id: 5, name: "くさ", color: "#80db9c"},
        {id: 6, name: "こおり", color: "#93f1e8"},
        {id: 7, name: "かくとう", color: "#cb5441"},
        {id: 8, name: "どく", color: "#bb82f1"},
        {id: 9, name: "じめん", color: "#bbb58b"},
        {id: 10, name: "ひこう", color: "#9fc7e9"},
        {id: 11, name: "エスパー", color: "#ef81d3"},
        {id: 12, name: "むし", color: "#c0cf84"},
        {id: 13, name: "いわ", color: "#bf9f84"},
        {id: 14, name: "ゴースト", color: "#a797cf"},
        {id: 15, name: "ドラゴン", color: "#7091ed"},
        {id: 16, name: "あく", color: "#5e95cd"},
        {id: 17, name: "はがね", color: "#93a1ab"},
        {id: 18, name: "フェアリー", color: "#f1b7f5"},
    ],
    GENDERS: [
        {id: 1, name: "♂"},
        {id: 2, name: "♀"},
        {id: 3, name: "不明"},
    ],
    COMMANDS: [
        {name: "たたかう", id: 1},
        {name: "どうぐ", id: 2},
        {name: "ポケモン", id: 3},
        {name: "にげる", id: 4},
    ],
    STATUS: [
        {id: 0, value: "h", name: "HP" },
        {id: 1, value: "a", name: "こうげき"},
        {id: 2, value: "b", name: "ぼうぎょ"},
        {id: 3, value: "c", name: "とくこう"},
        {id: 4, value: "d", name: "とくぼう"},
        {id: 5, value: "s", name: "すばやさ"},
    ],
    ABILITIES: [
        {id: 8, name: "すながくれ"},
        {id: 18, name: "もらいび"},
        {id: 22, name: "いかく"},
        {id: 24, name: "さめはだ"},
        {id: 45, name: "すなおこし"},
        {id: 49, name: "ほのおのからだ"},
        {id: 50, name: "にげあし"},
        {id: 76, name: "エアロック"},
        {id: 84, name: "かるわざ"},
        {id: 106, name: "ゆうばく"},
        {id: 120, name: "すてみ"},
        {id: 138, name: "ねつぼうそう"},
        {id: 159, name: "すなのちから"},
    ],
    NATURES: [
        {id: 1, name: "さみしがり", up: 1, down: 2},
        {id: 2, name: "いじっぱり", up: 1, down: 3},
        {id: 3, name: "やんちゃ", up: 1, down: 4},
        {id: 4, name: "ゆうかん", up: 1, down: 5},
        {id: 5, name: "ずぶとい", up: 2, down: 1},
        {id: 6, name: "わんぱく", up: 2, down: 3},
        {id: 7, name: "のうてんき", up: 2, down: 4},
        {id: 8, name: "のんき", up: 2, down: 5},
        {id: 9, name: "ひかえめ", up: 3, down: 1},
        {id: 10, name: "おっとり", up: 3, down: 2},
        {id: 11, name: "うっかりや", up: 3, down: 4},
        {id: 12, name: "れいせい", up: 3, down: 5},
        {id: 13, name: "おだやか", up: 4, down: 1},
        {id: 14, name: "おとなしい", up: 4, down: 2},
        {id: 15, name: "しんちょう", up: 4, down: 3},
        {id: 16, name: "なまいき", up: 4, down: 5},
        {id: 17, name: "おくびょう", up: 5, down: 1},
        {id: 18, name: "せっかち", up: 5, down: 2},
        {id: 19, name: "ようき", up: 5, down: 3},
        {id: 20, name: "むじゃき", up: 5, down: 4},
        {id: 21, name: "てれや", up: 1, down: 3},
    ],
}

//100:攻撃アクション,
//101:うけたダメージ量,
//102:急所にあてられた,
//104:いまひとつ、105:ばつぐん、106:こうかなし
//110:攻撃外し
//120:死亡

//200:交換