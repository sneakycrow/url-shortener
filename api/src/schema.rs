// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Uuid,
        target_url -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}
