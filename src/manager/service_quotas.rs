pub trait Manager {
    fn create(&self) -> String;
}

pub struct ServiceQuotas {
    pub telegram_id: i64,
    pub cpu: u8,
    pub memory: u8,
    pub disk_size: u8,
}

impl ServiceQuotas {
    pub async fn get_from_db(telegram_id: i64) -> Self {
        todo!()
    }
}

impl Manager for ServiceQuotas {
    fn create(&self) -> String {
        // save to db
        format!(
            "ServiceQuotas: telegram_id: {}, cpu: {}, memory: {}, disk_size: {}",
            self.telegram_id, self.cpu, self.memory, self.disk_size
        )
    }
}
