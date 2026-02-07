// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Int4,
        card_id -> Int4,
        user_id -> Int4,
        disabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        balance -> Float4,
    }
}

diesel::table! {
    scanner (id) {
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
        validity -> Timestamp,
        travelling_at -> Timestamp,
        created_at -> Timestamp,
        cost -> Float4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        user_id -> Varchar,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

diesel::allow_tables_to_appear_in_same_query!(card, scanner, ticket, user, useraccount,);
