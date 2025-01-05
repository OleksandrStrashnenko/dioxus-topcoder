use dioxus::fullstack::ServeConfigBuilder;
use dioxus::prelude::DioxusRouterExt;
use crate::components::app::App;

#[cfg(feature = "server")]
#[allow(dead_code)]
pub(crate) async fn launch_server() {
    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();

    // And launch it!
    // axum::Server::bind(&socket_addr).serve(router).await
    let listener = match tokio::net::TcpListener::bind(socket_addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Error: {}", e);
            return
        }
    };
    axum::serve(listener, router).await.unwrap();
}