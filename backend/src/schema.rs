// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Int4,
        card_id -> Int4,
        user_id -> Int4,
        balance -> Float4,
    }
}

diesel::table! {
    scannner (id) {
        id -> Int4,
        scanner_id -> Int4,
    }
}

diesel::table! {
    ticket (id) {
        id -> Int4,
        card_id -> Int4,
        user_id -> Int4,
        origin -> Varchar,
        destination -> Varchar,
        validity -> Time,
        travel_date -> Date,
        travel_time -> Time,
        cost -> Float4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        user_id -> Varchar,
        username -> Varchar,
        creation -> Int4,
    }
}

diesel::table! {
    useraccount (id) {
        id -> Int4,
        user_id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(card, scannner, ticket, user, useraccount,);
