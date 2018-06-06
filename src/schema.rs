table! {
    item (id) {
        id -> Nullable<Integer>,
        name -> Text,
        price -> Integer,
        desc -> Nullable<Text>,
        valid -> Integer,
    }
}

table! {
    keymap (id) {
        id -> Nullable<Integer>,
        code -> SmallInt,
        key -> SmallInt,
        item_id -> Integer,
    }
}

table! {
    sell (id) {
        id -> Nullable<Integer>,
        item_id -> Integer,
        tz -> BigInt,
        sold -> Nullable<Integer>,
    }
}

joinable!(keymap -> item (item_id));
joinable!(sell -> item (item_id));

//allow_tables_to_appear_in_same_query!(
//    item,
//    keymap,
//    sell,
//);
