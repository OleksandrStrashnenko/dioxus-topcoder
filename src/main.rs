use dioxus::events::keyboard_types::webdriver::KeyInputState;
use dioxus::logger::tracing::log;
// use axum::ServiceExt;
use dioxus::prelude::*;
use rusqlite::params;
use serde_json::{json, Value};

// use serde_json::{json, Value};
#[cfg(feature = "server")]
mod server;
mod translate;

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
        Hero {}
    }
}

// #[server]
async fn save_translation(src: String, translated: String) -> Result<(), ServerFnError> {
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
    let mut trans = use_signal(|| "".to_string());
    let handle_key_down = move |evt: Event<FormData>| async move {
        let src = evt.data().value();
        if src.is_empty() {
            return;
        }
        let query_result: Result<String, rusqlite::Error> = DB.with(
            |f| f.query_row(
                "SELECT (translated) FROM dictionary WHERE original == ?1",
                params![src],
                |row| {
                    row.get(0)
                })
        );
        match query_result {
            Ok(translated) => trans.set(translated),
            Err(e) => {
                let translation = match translate::translate(&src).await {
                    Some(translation) => {
                        if let Some(translated) = translation.translated.as_str() {
                            if let Err(err) = save_translation(src, String::from(translated)).await {
                                eprintln!("Error: {err}");
                            };
                            trans.set(translated.into());
                        }

                        translation
                    },
                    None => { return; }
                };
            }
        }

    };
    rsx! {
        div {
            id: "working-panel",
            input { id: "source-text-to-translate", oninput: handle_key_down }
            div { id: "translated", "{trans}"}
        }
    }
}
