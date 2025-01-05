use dioxus::events::keyboard_types::webdriver::KeyInputState;
use dioxus::logger::tracing::log;
// use axum::ServiceExt;
use dioxus::prelude::*;
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
// thread_local! {
//     pub static DB: rusqlite::Connection = {
//         let conn = rusqlite::Connection::open("dictionary.db").expect("Failed to open database");
//         conn.execute_batch(
//             "CREATE TABLE IF NOT EXISTS dictionary (
//                 id INTEGER PRIMARY KEY,
//                 original TEXT NOT NULL,
//                 translated TEXT NOT NULL
//             ); 
//             "
//         ).unwrap();

//         conn
//     }
// }

// #[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[server]
async fn get_translation(src: String, translated: String) -> Result<(), ServerFnError> {
    // DB.with(|f| f.execute("INSERT INTO dictionary(original, translated) VALUES (?1, ?2)", &[&src, &translated]))?;
    Ok(())
}

const RPC_ID: &str = "MkEWBc";

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
    let mut handle_key_down = move |evt: Event<FormData>| async move {
        let client = reqwest::Client::new();
        let src = evt.data().value();
        let response = client
            .post("https://translate.google.com/_/TranslateWebserverUi/data/batchexecute?rpcids=MkEWBc&bl=boq_translate-webserver_20201207.13_p0&soc-app=1&soc-platform=1&soc-device=1&rt=c")
            // .post("httdddp://httpbin.org/post")
            // .post("https://www.google.com/")
            // .body("sl=de&tl=en&q=Apple")
            .body(format!("f.req={}", build_rpc_request(src.as_str(), "uk", "auto")))
            .header("host", "translate.google.com")
            .header("accept", "*/*")
            .header("accept-encoding", "gzip, deflate")
            .header("connection", "keep-alive")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .header("referer", "https://translate.google.com")
            // .header("content-length", "5")
            .header("content-type", "application/x-www-form-urlencoded")
            // .header("x-goog-user-project", "rugged-sunbeam-443912-c5")
            // .header("Content-Type", "application/json; charset=utf-8") //"application/x-www-form-urlencoded;charset=utf-8'")
            // .header("Content-Type", "application/x-www-form-urlencoded;charset=utf-8'")
            // .header("Access-Control-Allow-Origin", "*")
            .send()
            .await;
        // response.text().await.unwrap_or("Could send request".to_string());
        let resp  = match response {
            Ok(response) => {
                let result = response.text().await;
                let result = match result {
                    Ok(text) => {
                        println!("result: {text}");
                        text
                    },
                    Err(e) => {
                        println!("error while fetching result: {e}");
                        e.to_string()
                    }
                };
                result
            },
            Err(error) => {
                let result = format!("{}", error.to_string());
                println!("error: {result}");
                result
            }
        };
        let resp = match serde_json::from_str::<Value>(
            match resp.split('\n').find(|line| line.contains(RPC_ID)) {
                Some(line) => line,
                None => {
                    eprintln!("Could not find RPC_ID.");
                    return;
                } }) {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Could not deserialize input. {}", e);
                return;
            }
        };
        let parsed = match serde_json::from_str::<Value>(
            match resp[0][2].as_str() {
                Some(text) => text,
                None => {
                    eprintln!("Could not find JSON string.");
                    return;
                }
            }
        ) {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Could not deserialize input. {}", e);
                return;
            }
        };
        let translated_parts: Value = serde_json::from_value(parsed[1][0][0][5].clone()).unwrap();
        let translated = translated_parts[0][0].clone();
        let pronunciation: Result<Value, _> = serde_json::from_value(parsed[1][0][0][1].clone());
        println!("translated: {translated}");
        if let Err(err) = get_translation(src, trans.to_string()).await {
            eprintln!("Error: {err}");
        };
        trans.set(match translated.as_str() {
            Some(res) => res.to_string(),
            None => "Error occured".to_string()
        });
    };
    rsx! {
        div {
            id: "working-panel",
            input { id: "source-text-to-translate", oninput: handle_key_down }
            div { id: "translated", "{trans}"}
        }
    }
}
