// @generated automatically by Diesel CLI.

diesel::table! {
    service_quotas (id) {
        id -> Nullable<Integer>,
        telegram_id -> Integer,
        cpu -> Integer,
        memory -> Integer,
        disk_size -> Integer,
    }
}
