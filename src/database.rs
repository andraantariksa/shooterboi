#[cfg(target_arch = "wasm32")]
use gluesql::prelude::MemoryStorage as Storage;
#[cfg(not(target_arch = "wasm32"))]
use gluesql::prelude::SledStorage as Storage;
use gluesql::prelude::{Glue, MemoryStorage, Payload};
#[cfg(not(target_arch = "wasm32"))]
use gluesql::sled::IVec as Debug;
#[cfg(target_arch = "wasm32")]
use gluesql::storages::memory_storage::Key as Debug;

pub struct Database {
    pub glue: Glue<Debug, Storage>,
}

impl Database {
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let storage = Storage::new("db").unwrap();
        #[cfg(target_arch = "wasm32")]
        let storage = Storage::default();
        Self {
            glue: Glue::new(storage),
        }
    }

    pub fn init(&mut self) {
        let query = "CREATE TABLE IF NOT EXISTS settings (
    audio_volume FLOAT NOT NULL,
    maximum_raymarch_step INTEGER NOT NULL,
)";
        self.glue.execute(query);

        let query = "SELECT * FROM settings";
        let output = self.glue.execute(query).unwrap();
        match output {
            Payload::Select { ref rows, .. } => {
                if rows.is_empty() {
                    let query = "INSERT INTO settings VALUES (1.0, 50)";
                    self.glue.execute(query);
                }
            }
            _ => {}
        };
    }
}
