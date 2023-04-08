// @generated automatically by Diesel CLI.

diesel::table! {
    visits (id) {
        id -> Uuid,
        ip_address -> Nullable<Varchar>,
        page_visited -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
