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
        {id: 3, name: ""},
    ],
    COMMANDS: [
        {name: "たたかう", id: 1},
        {name: "どうぐ", id: 2},
        {name: "ポケモン", id: 3},
        {name: "にげる", id: 4},
    ],

}


//100:攻撃アクション, 101:うけたダメージ量,
//102:急所にあてられた,
//104:いまひとつ、105:ばつぐん、106:こうかなし