// @generated automatically by Diesel CLI.

diesel::table! {
    surveys (id) {
        id -> Uuid,
        question -> Varchar,
        answer -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    visits (id) {
        id -> Uuid,
        ip_address -> Nullable<Varchar>,
        page_visited -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    waitlists (id) {
        id -> Uuid,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(surveys, visits, waitlists,);
