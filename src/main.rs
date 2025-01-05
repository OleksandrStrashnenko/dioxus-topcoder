use dioxus::prelude::*;
use dioxus::desktop::*;
use crate::components::app::App;

#[cfg(feature = "server")]
mod server;
mod translate;
mod components;

#[cfg(feature = "server")]
#[cfg(not(feature = "desktop"))]
fn main() {
    // dioxus::launch(App);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { server::launch_server().await; });
}

#[cfg(feature = "desktop")]
fn main() {
    LaunchBuilder::new()
        .with_cfg(
            if cfg!(debug_assertions) { 
                    Config::new() 
                } else {
                    Config::new()
                        .with_menu(None)
                }
        )
        .launch(App);
}



// #[cfg(feature = "server")]
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






