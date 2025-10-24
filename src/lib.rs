pub mod components;
pub mod quiz_service;
pub mod translate;

thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("dictionary.db").expect("Failed to open database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dictionary (
                original TEXT UNIQUE NOT NULL,
                translated TEXT NOT NULL
            );
            "
        ).unwrap();

        conn
    }
}
