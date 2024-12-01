diesel::table! {
    service_quotas (id) {
        id -> Int4,
        telegram_id -> Int4,
        cpu -> Int4,
        memory -> Int4,
        disk_size -> Int4,
    }
}
