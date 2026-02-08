// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Int4,
        user_id -> Int4,
        card_id -> Varchar,
        disabled -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        balance -> Float4,
    }
}

diesel::table! {
    scanner (id) {
        id -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    ticket (id) {
        id -> Int4,
        card_id -> Int4,
        user_id -> Int4,
        origin -> Nullable<Varchar>,
        destination -> Nullable<Varchar>,
        validity -> Nullable<Timestamptz>,
        travelling_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        cost -> Float4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    useraccount (id) {
        id -> Int4,
        user_id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(card -> user (user_id));
diesel::joinable!(ticket -> card (card_id));
diesel::joinable!(ticket -> user (user_id));
diesel::joinable!(useraccount -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(card, scanner, ticket, user, useraccount,);
