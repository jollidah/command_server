use once_cell::sync::Lazy;
use std::sync::Mutex;

static TEST_DB: Lazy<Mutex<Option<Arc<RocksDb>>>> = Lazy::new(|| Mutex::new(None));

pub fn get_rocks_db() -> Arc<RocksDb> {
    if cfg!(test) {
        let mut db = TEST_DB.lock().unwrap();
        if db.is_none() {
            let rocks_db = Arc::new(RocksDb::new().expect("Failed to initialize RocksDB"));
            *db = Some(rocks_db.clone());
            rocks_db
        } else {
            db.as_ref().unwrap().clone()
        }
    } else {
        static DB: Lazy<Arc<RocksDb>> = Lazy::new(|| {
            Arc::new(RocksDb::new().expect("Failed to initialize RocksDB"))
        });
        DB.clone()
    }
} 