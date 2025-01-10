use serde_json::Value;
use crate::{build_rpc_request};

const RPC_ID: &str = "MkEWBc";
pub(crate) struct Translation {
    pub(crate) translated: Value
}

pub(crate) async fn translate(src: &String) -> Option<Translation> {
    let client = reqwest::Client::new();
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
                return None;
            } }) {
        Ok(line) => line,
        Err(e) => {
            eprintln!("Could not deserialize input. {}", e);
            return None;
        }
    };
    let parsed = match serde_json::from_str::<Value>(
        match resp[0][2].as_str() {
            Some(text) => text,
            None => {
                eprintln!("Could not find JSON string.");
                return None;
            }
        }
    ) {
        Ok(line) => line,
        Err(e) => {
            eprintln!("Could not deserialize input. {}", e);
            return None;
        }
    };
    let translated_parts: Value = serde_json::from_value(parsed[1][0][0][5].clone()).unwrap();
    let translated = translated_parts[0][0].clone();
    let pronunciation: Result<Value, _> = serde_json::from_value(parsed[1][0][0][1].clone());
    println!("translated: {translated}");
    Translation {translated}.into()
}