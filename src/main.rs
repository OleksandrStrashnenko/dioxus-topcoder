use dioxus::prelude::*;
use dioxus::desktop::*;
use crate::components::app::App;
use dioxus::desktop::tao;
use dioxus::logger::tracing::Instrument;

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
    // let icon = tao::window::Icon::from_rgba(include_bytes!("../assets/favicon.ico").to_vec(), 256, 256).unwrap();
    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_menu(None)
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






