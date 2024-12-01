use crate::database::schema::service_quotas;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::service_quotas)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ServiceQuotas {
    pub id: i32,
    pub telegram_id: i32,
    pub cpu: i32,
    pub memory: i32,
    pub disk_size: i32,
}

#[derive(Insertable)]
#[diesel(table_name = service_quotas)]
pub struct NewServiceQuotas<'a> {
    pub telegram_id: &'a i32,
    pub cpu: &'a i32,
    pub memory: &'a i32,
    pub disk_size: &'a i32,
}
