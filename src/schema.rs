table! {
    api_user_secret (id) {
        id -> Integer,
        app_code -> Text,
        app_secret -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    admin (id) {
        id -> Integer,
        username -> Text,
        passwd -> Text,
        salt -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    admin_access_token (id) {
        id -> Integer,
        admin_id -> Integer,
        access_token -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    last_insert (id) {
        id -> Integer,
    }
}
