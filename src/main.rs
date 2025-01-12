use dioxus::events::keyboard_types::webdriver::KeyInputState;
use dioxus::html::meta::charset;
use dioxus::logger::tracing::log;
// use axum::ServiceExt;
use dioxus::prelude::*;
use rusqlite::params;
use serde_json::{json, Value};

// use serde_json::{json, Value};
#[cfg(feature = "server")]
mod server;
mod translate;
mod components;
use components::history::HistoryItem;
use crate::translate::translate_from_db_or_google;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
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
    dioxus::launch(App);
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

// #[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Meta { charset: "utf-8" }
        Hero {}
    }
}

// #[server]
async fn save_translation(src: &String, translated: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dictionary(original, translated) VALUES (?1, ?2) ON CONFLICT(original) DO NOTHING;", &[&src, &translated]))?;
    Ok(())
}


fn build_rpc_request(text: &str, dest: &str, src: &str) -> String {
    json!([[
        [
            "MkEWBc",
            format!("[[{text}, {src}, {dest}, true], [null]]").as_str(),
            "null",
            "generic"
        ]
    ]]).to_string()
}
//
#[component]
pub fn Hero() -> Element {
    let mut trans = use_signal(|| "Translated".to_string());
    let mut history_list: Signal<Vec<HistoryItem>> = use_signal(|| vec![]);
    let handle_key_down = move |evt: Event<FormData>| async move {
        let src = evt.data().value();
        match translate_from_db_or_google(&src).await {
            Some(translation) => {
                trans.set(translation.clone());
                history_list.push(HistoryItem::new(src.clone(), translation))
            },
            None => {
                trans.set("Translation".to_string());
            }
        }
    };
    rsx! {
        div {
            id: "working-panel",
            input {
                id: "source-text-to-translate",
                class: "working-div",
                background_color: "white",
                oninput: handle_key_down
            }
            div { class: "working-div",
                id: "translated",
                "{trans}"
            }
        }
        div {
            table {
                for item in history_list.iter() {
                    tr {
                        td { "{item.src()}" }
                        td { "{item.translated()}" }
                    }
                }
            }
        }
    }
}
