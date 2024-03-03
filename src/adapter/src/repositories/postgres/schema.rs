// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        tags -> Nullable<Array<Nullable<Text>>>,
        created_on -> Timestamp,
    }
}
