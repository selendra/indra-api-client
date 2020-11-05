table! {
    login_history (id) {
        id -> Int4,
        user_id -> Uuid,
        login_timestamp -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(
    login_history,
    users,
);
