use crate::components::app::App;
use dioxus::desktop::*;
use dioxus::prelude::*;

mod components;
#[cfg(feature = "server")]
mod server;
mod translate;

#[cfg(feature = "server")]
#[cfg(not(feature = "desktop"))]
fn main() {
    // dioxus::launch(App);
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        server::launch_server().await;
    });
}

#[cfg(feature = "desktop")]
fn main() {
    let drag_disabled = true;
    LaunchBuilder::new()
        .with_cfg(if cfg!(debug_assertions) {
            Config::new().with_disable_drag_drop_handler(drag_disabled)
        } else {
            Config::new()
                .with_menu(None)
                .with_disable_drag_drop_handler(drag_disabled)
        })
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
