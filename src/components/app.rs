use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::document;
use dioxus::hooks::use_context_provider;
use dioxus::prelude::*;
use crate::components::history::{HistoryBar, HistoryItem};
use crate::components::working_panel::working_panel::WorkingPanel;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const WORKING_PANEL_CSS: Asset = asset!("/assets/working-panel.css");
// const FAVICON_TYPE = html::image::r#type("image/x-icon");
const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap-5.3.5/dist/css/bootstrap.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

// #[component]
#[allow(non_snake_case)]
#[component]
pub fn App() -> Element {
    // let mut history_list: Signal<Vec<HistoryItem>> = use_signal(|| vec![]);
    use_context_provider(|| Signal::<Vec<HistoryItem>>::new(vec![]));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: WORKING_PANEL_CSS }
        document::Meta { charset: "utf-8" }
        div { id: "body",
            WorkingPanel {}
            HistoryBar {}
        }
    }
}