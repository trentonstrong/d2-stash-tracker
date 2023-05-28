// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Integer,
        name -> Text,
        level -> Integer,
        class -> Text,
        is_expansion -> Bool,
        has_died -> Bool,
        is_hardcore -> Bool,
        is_ladder -> Bool,
        saved_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
