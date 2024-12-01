pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewServiceQuotas, ServiceQuotas};
use std::env;

use crate::database::schema::service_quotas;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_service_quotas(conn: &mut SqliteConnection, telegram_id: i32) -> ServiceQuotas {
    let new_service_quotas = NewServiceQuotas {
        telegram_id: &telegram_id,
        cpu: &6,
        memory: &8,
        disk_size: &50,
    };

    diesel::insert_into(service_quotas::table)
        .values(&new_service_quotas)
        .returning(ServiceQuotas::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
